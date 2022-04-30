<script>
    // Reusable
    import Loading from "../reusable/Loading.svelte";
    import Error from "../reusable/Error.svelte";
    import Success from "../reusable/Success.svelte";
    // Stores & APIs
    import { theme } from "../stores";
    import { fade, fly } from "svelte/transition";
    // Components
    import StageName from "../components/StageName.svelte";
    import StagePassword from "../components/StagePassword.svelte";

    let user = {
        id: 114514,
        name: "张三",
        avatar: "https://avatars0.githubusercontent.com/u/114514?s=460&v=4",
    }; // 存储用户信息

    let stage; // name: 姓名, qr: 二维码, password: 密码, error: 错误, loading: 加载, success: 成功, code: 验证码

    // error页面展示的数据
    let error_data = {};

    // any -> error
    function error(reason, back_to) {
        stage = "error";
        error_data = {
            reason,
            back_to,
        };
    }

    // any -> loading
    let loading_data = {};
    function loading() {
        loading_data = {
            logs: [],
        };
        stage = "loading";
    }

    function log(type, message) {
        loading_data.logs = [...loading_data.logs, { type, message }];
    }

    import {
        authenticate_session,
        get_user_by_name,
        get_user_by_email,
        get_qrcode,
        login
    } from "../apis";

    function is_email(content) {
        return /^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/.test(
            content
        );
    }

    // name -> password / name -> error
    function load_name_or_email(e) {
        console.log(e.detail);
        if (e.detail.length === 0) {
            error("请输入邮箱或姓名", "name");
            return;
        }

        loading("password");
        log("info", "校验客户端...");

        authenticate_session(undefined)
            .then(() => {
                log("success", "客户端校验成功");
                log("info", "获取用户信息...");

                (is_email(e.detail) ? get_user_by_email : get_user_by_name)(
                    e.detail
                )
                    .then((_user) => {
                        user = _user;
                        log("success", "用户信息获取成功");
                        stage = "password";
                    })
                    .catch((e) => {
                        if (e.name === "user does not exist") {
                            log("error", "用户不存在");
                            error("用户不存在", "name");
                            name = "";
                        } else {
                            log("error", "获取用户信息失败");
                            error("获取用户信息失败", "name");
                        }
                    });
            })
            .catch((e) => {
                if (e.name === "verification code required") {
                    log("error", "服务器反馈需要验证码");
                    stage = "code";
                }
            });
    }

    // password -> success / password -> error
    function loading_login(e) {
        console.log(e.detail);
        if (e.detail.length === 0) {
            error("请输入密码", "password");
            return;
        }

        loading();
        log("info", "尝试登录...");
        login(user.id, e.detail).then(() => {
            log("success", "登录成功");
            stage = "success";
        }).catch((e) => {
            if (e.name === "wrong password") {
                log("error", "密码错误");
                error("密码错误", "password");
            } else {
                log("error", "登录失败");
                error("登录失败", "name");
            }
        });
    }

    // name -> loading
    let qr_link;
    function loading_qr() {
        loading();
        log("info", "获取登录二维码...");
        get_qrcode().then((link) => {
            log("success", "获取登录二维码成功!");
            qr_link = link;
            stage = "qr";
        });
    }

    import { init_session } from "../apis";
    loading();
    log("info", "同步客户端身份数据...");
    init_session()
        .then((session) => {
            log("success", "客户端身份数据同步成功!");
            if (session.login) {
                // 已经登录了
                stage = "success";
                user = session.user;
            } else {
            
                stage = "name";
            }
        })
        .catch((e) => {
            log("error", "客户端身份数据同步失败!");
            error("无法与服务器同步客户端身份", "name");
        });
</script>

<div
    class="main"
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
    in:fade={{ delay: 300 }}
>
    {#if stage === "name"}
        <StageName on:next={load_name_or_email} on:wechat={loading_qr} />
    {/if}
    {#if stage === "password"}
        <StagePassword {user} on:next={loading_login} />
    {/if}
    {#if stage === "loading"}
        <div in:fly={{ x: 40 }}>
            <div style="height: 40px;" />
            <Loading logs={loading_data.logs} />
            <div style="height: 20px;" />
        </div>
    {/if}
    {#if stage === "qr"}
        <img src={qr_link} alt={qr_link} />
    {/if}
    {#if stage === "error"}
        <div in:fly={{ x: 40 }}>
            <Error
                reason={error_data.reason}
                back_text="返回"
                on:click={() => {
                    stage = error_data.back_to;
                }}
            />
        </div>
    {/if}
    {#if stage === "success"}
        <div in:fly={{ x: 40 }}>
            <Success
                reason="登录成功"
                back_text="返回"
                on:click={() => {
                    stage = "name";
                }}
            />
        </div>
    {/if}
</div>
