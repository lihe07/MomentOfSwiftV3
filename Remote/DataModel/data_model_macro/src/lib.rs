extern crate proc_macro;

use quote::quote;


#[proc_macro_derive(ById)]
pub fn by_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let async_trait: syn::Attribute = syn::parse_quote!(#[async_trait::async_trait]);

    let impl_block = quote! {
        #async_trait
        impl #impl_generics crate::collections::ById for #name #ty_generics #where_clause {

            /// 根据id获取一个实例
            async fn by_id<T: Into<String> + Send>(repo: &mongodm::Repository<Self>, id: T) -> Option<Self> {
                // 默认实现
                // 先从缓存中查找
                let id = id.into();
                if let Some(data) = crate::collections::CACHE.get(&id).await {
                    Some(bson::from_slice(data.as_slice()).unwrap())
                } else {
                    // 如果缓存中没有，则从数据库中查找
                    let result = repo.find_one(bson::doc! {
                        "id": id.to_owned()
                    }, None).await;
                    if let Ok(option) = result {
                        if let Some(doc) = option {
                            // 将查找到的数据放入缓存
                            crate::collections::CACHE.set(id, bson::to_vec(&doc).unwrap(), None).await;
                            return Some(doc);
                        }
                    }
                    None
                }
            }
            /// 根据多个id查找多个实体
            async fn by_ids<T: Into<Vec<String>> + Send>(repo: &mongodm::Repository<Self>, ids: T) -> Vec<Self> {
                // 默认实现 循环调用 by_id
                let ids = ids.into();
                let mut result = Vec::new();
                for id in ids {
                    if let Some(doc) = Self::by_id(repo, id).await {
                        result.push(doc);
                    }
                }
                result
            }
            /// 同步到缓存
            async fn sync(&self) {
                // 默认实现
                crate::collections::CACHE.set(self.id.to_string(), bson::to_vec(&self).unwrap(), None).await;
            }
        }
    };
    proc_macro::TokenStream::from(impl_block)
}


enum Attribute {
    Hidden,
    Expand {
        model: syn::Ident,
    },
    None,
}


fn parse_attribute(attrs: Vec<syn::Attribute>) -> Attribute {
    for attr in attrs {
        if attr.path.segments.len() == 1 {
            let name = attr.path.segments[0].ident.to_string();
            if name == "hidden" {
                return Attribute::Hidden;
            } else if name == "expand" {
                for token in attr.tokens.clone() {
                    if let quote::__private::TokenTree::Group(group) = token {
                        if let quote::__private::Delimiter::Parenthesis = group.delimiter() {
                            let mut iter = group.stream().into_iter();
                            let key = match iter.next() {
                                Some(quote::__private::TokenTree::Ident(ident)) => ident,
                                _ => panic!("expand 属性必须为 #[expand(ident=ident)] 格式")
                            };
                            if let Some(quote::__private::TokenTree::Punct(punct)) = iter.next() {
                                if punct.as_char() != '=' {
                                    panic!("expand 属性必须为 #[expand(ident=ident)] 格式")
                                }
                            } else {
                                panic!("expand 属性必须为 #[expand(ident=ident)] 格式")
                            }
                            let value = match iter.next() {
                                Some(quote::__private::TokenTree::Ident(ident)) => ident,
                                _ => panic!("expand 属性必须为 #[expand(ident=ident)] 格式")
                            };

                            if key.to_string() != "model" {
                                panic!("expand 目前只支持 model 属性")
                            }
                            return Attribute::Expand {
                                model: value,
                            };
                        }
                    }
                }
                return Attribute::None;
            }
        }
    }
    Attribute::None
}

fn parse_type(ty: &syn::Type) -> String {
    if let syn::Type::Path(path) = ty {
        if path.path.segments.len() == 1 {
            return path.path.segments[0].ident.clone().to_string();
        } else {
            // 取最末尾的
            return path.path.segments[path.path.segments.len() - 1].ident.clone().to_string();
        }
    }
    panic!("类型必须是一个标识符");
}


#[proc_macro_derive(ToJson, attributes(hidden, expand))]
pub fn to_json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // 流程:
    // 1. 列出所有的字段
    // 2. 列出所有hidden的字段
    // 在to_json中，先将自身转换为serde_json::Value，然后去掉hidden和expand的字段
    // 4. 获取expand的字段的属性参数 也就是方法名 如 #[expand(User::by_id)] 中的 User::by_id
    // 5. 调用方法 之后再调用它的to_json_without_expand
    // 6. 将结果返回
    let name = &input.ident;
    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let _struct = if let syn::Data::Struct(data) = &input.data {
        data.to_owned()
    } else {
        panic!("#[derive(ToJson)] 仅支持应用于 struct");
    };
    //
    let mut datetime_fields = Vec::new(); // 将所有bson::DateTime类型的字段名称放入这个vec中
    let mut hidden_fields = Vec::new();
    let mut expand_fields = Vec::new();

    for field in _struct.fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let field_type = parse_type(&field.ty);
        let attribute = parse_attribute(field.attrs);
        match attribute {
            Attribute::Hidden => {
                hidden_fields.push(field_name);
                continue;
            }
            Attribute::Expand { model } => {
                expand_fields.push((field_name, model));
                continue;
            }
            _ => {}
        }
        if field_type == "DateTime" {
            datetime_fields.push(field_name);
        }
    }

    let mut remove_hidden_fields = proc_macro2::TokenStream::new();
    for field_name in hidden_fields {
        remove_hidden_fields.extend(quote! {
            object.remove(#field_name);
        });
    }
    let mut format_datetime_fields = proc_macro2::TokenStream::new();
    for field_name in datetime_fields {
        format_datetime_fields.extend(quote! {
            *value.get_mut(#field_name).unwrap() = serde_json::json!(value.get(#field_name).unwrap().get("$date").unwrap().get("$numberLong").unwrap().as_str().unwrap().parse::<i64>().unwrap());
        });
    }

    let mut find_expand_fields = proc_macro2::TokenStream::new();
    for (field_name, model) in expand_fields {
        find_expand_fields.extend(quote! {
            if let serde_json::Value::String(id) = object.get(#field_name).unwrap() {
                let repo = db.repository::<#model>();
                let model = #model::by_id(&repo, id).await.unwrap();
                object.insert(#field_name.to_string(), model.to_json_without_expand());
            }
        });
    }


    let impl_block = quote! {
        #[async_trait::async_trait]
        impl #impl_generics crate::collections::ToJson for #name #ty_generics #where_clause {
            async fn to_json(self, db: &mongodm::mongo::Database) -> serde_json::Value {
                use crate::collections::ById;
                let mut value = self.to_json_without_expand();
                if let serde_json::Value::Object(ref mut object) = value {
                    #find_expand_fields
                }
                value
            }

            fn to_json_without_expand(self) -> serde_json::Value {
                // 先将自身转换为serde_json::Value，然后去掉hidden
                let mut value = serde_json::to_value(self).unwrap();
                // 去掉hidden的字段
                if let serde_json::Value::Object(ref mut object) = value {
                    #remove_hidden_fields
                }
                #format_datetime_fields

                value
            }
        }
    };
    impl_block.into()
}