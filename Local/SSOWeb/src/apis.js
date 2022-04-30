// 获取并初始化session
export const init_session = () => {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            if (Math.random() > 0.1) {
                // resolve({
                //     fingerprint: "abcdefg",
                //     permissions: [],
                //     login: false
                // });
                resolve({
                    fingerprint: "abcdefg",
                    permissions: [],
                    login: false,
                    // login: true,
                    // user: {
                    //     id: 114514,
                    //     name: "张三",
                    //     avatar: "https://avatars0.githubusercontent.com/u/114514?v=4"
                    // }
                });
            } else {
                reject();
            }

        }, Math.random() * 1000);
    });
}

export const get_user_by_name = (name) => {
    // 虚拟一些假数据 和延迟
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            if (name === "张三") {
                resolve({
                    id: 114514,
                    name: "张三",
                    avatar: "https://avatars0.githubusercontent.com/u/114514?v=4"
                });
            } else {
                reject({
                    name: "user does not exist",
                    message: "用户不存在"
                });
            }
        }, Math.random() * 1000);
    });
}


export const get_user_by_email = (email) => {
    // 虚拟一些假数据 和延迟
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            if (email === "li@imlihe.com") {
                resolve({
                    id: 114514,
                    name: "张三",
                    avatar: "https://avatars0.githubusercontent.com/u/114514?v=4"
                });
            } else {
                reject({
                    name: "user does not exist",
                    message: "用户不存在"
                });
            }
        }, Math.random() * 1000);
    });
}

// 申请验证会话的安全性
// 没有被验证的会话不能获取用户信息
// 同一个IP在10分钟内 第二次申请验证的时候 会提出需要验证码
export const authenticate_session = (code = undefined) => {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            resolve(); // 会话已经被验证
        }, Math.random() * 1000);
    });
}

// 获取登录用的二维码
export const get_qrcode = () => {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            resolve("https://login.weixin.qq.com/qrcode/Qb67bL-KCA==");
        }, Math.random() * 1000);
    });
}


// 获取推荐的背景图片
export const get_background = () => {
    return new Promise((resolve, reject) => {
        resolve({
            url: "https://pic-1301810764.cos.ap-beijing.myqcloud.com/bg.jpg",
            likes: 3,
            author: "张三"
        })
    });
}

// 登录
export const login = (id, password) => {
    return new Promise((resolve, reject) => {
        if (id === 114514 && password === "114514") {
            resolve()
        } else {
            reject({
                name: "wrong password",
                message: "密码错误"
            })
        }
    })
}