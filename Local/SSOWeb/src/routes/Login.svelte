<script>
    // =============
    // |  外部内容   |
    // =============
    // Reusable
    import Loading from "../reusable/Loading.svelte";
    import Error from "../reusable/Error.svelte";
    // Components
    import StageName from "../components/StageName.svelte";
    import StagePassword from "../components/StagePassword.svelte";
    // APIs & Stores
    import {theme} from "../stores";
    import {fade, fly} from "svelte/transition";
    import {authenticate_session, get_user_by_name, get_users_by_email, get_qrcode, init_session, login} from "../apis";


    // =============
    // |  数据定义   |
    // =============
    let user; // 存储用户信息
    let users; // select阶段使用的用户信息
    let stage = "name"; // name: 姓名, qr: 二维码, password: 密码, error: 错误, loading: 加载, success: 成功, code: 验证码
    let error_data = {}; // error阶段展示的数据
    let loading_data = {}; // loading阶段展示的数据
    let qr_link; // qr阶段的二维码


    // =============
    // |  函数定义   |
    // =============
    // any -> error
    function error(reason, back_to) {
        stage = "error";
        error_data = {
            reason,
            back_to,
        };
    }

    // any -> loading
    function loading() {
        loading_data = {
            logs: [],
        };
        stage = "loading";
    }

    // loading阶段时写入新的日志
    function log(type, message) {
        loading_data.logs = [...loading_data.logs, {type, message}];
    }


    // 判断是否为邮箱
    function is_email(content) {
        return content.match(/^\w+@\w+\.\w+$/i)
    }

    // name -> password / name -> error
    function load_name_or_email(e) {
        if (e.detail.length === 0) {
            error("请输入邮箱或姓名", "name");
            return;
        }
        let name_or_email = e.detail;

        loading("password");
        log("info", "校验客户端...");

        authenticate_session(undefined)
            .then(() => {
                log("success", "客户端校验成功");
                log("info", "获取用户信息...");

                if (is_email(name_or_email)) {
                    get_users_by_email(name_or_email)
                        .then((_users) => {
                            if (_users.length === 0) {
                                // 没有找到任何用户
                                log("error", "用户不存在");
                                error("用户不存在", "name");
                                name_or_email = "";
                            } else if (_users.length === 1) {
                                // 只有一个用户
                                log("success", "通过邮箱找到了 1 个用户");
                                user = _users[0]
                                log("success", "用户信息获取成功")
                            } else {
                                // 有多个用户
                                log("success", "通过邮箱找到了 " + _users.length + " 个用户");

                            }
                        })
                        .catch((_) => {
                            log("error", "获取用户信息失败");
                            error("获取用户信息失败", "name");
                        })
                } else {
                    get_user_by_name(name_or_email)
                        .then((_user) => {
                            user = _user;
                            log("success", "用户信息获取成功");
                            stage = "password";
                        })
                        .catch((e) => {
                            if (e.name === "user does not exist") {
                                log("error", "用户不存在");
                                error("用户不存在", "name");
                                name_or_email = "";
                            } else {
                                log("error", "获取用户信息失败");
                                error("获取用户信息失败", "name");
                            }
                        });
                }


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
    function loading_qr() {
        loading();
        log("info", "获取登录二维码...");
        get_qrcode().then((link) => {
            log("success", "获取登录二维码成功!");
            qr_link = link;
            stage = "qr";
        });
    }


    // 初始化
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
        <StageName on:next={load_name_or_email} on:wechat={loading_qr}/>
    {/if}
    {#if stage === "password"}
        <StagePassword {user} on:next={loading_login}/>
    {/if}
    {#if stage === "loading"}
        <div in:fly={{ x: 40 }}>
            <div style="height: 40px;"/>
            <Loading logs={loading_data.logs}/>
            <div style="height: 20px;"/>
        </div>
    {/if}
    {#if stage === "qr"}
        <img src={qr_link} alt={qr_link}/>
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

<style>
    .split {
        display: flex;
        align-items: center;
    }

    .split > div {
        flex: 1;
        height: 1px;
        transition: background-color 0.2s;
    }

    .split > span {
        line-height: 100%;
        margin: 5px;
        font-size: 14px;
        letter-spacing: 2px;
        transition: color 0.2s;
        font-family: var(--font-sans-serif);
        text-align: center;
    }

    /* 深浅切换 */
    .dark .split > div {
        background-color: rgba(46, 46, 64, 1);
    }

    .light .split > div {
        background-color: rgba(236, 236, 236, 1);
    }

    /* 深浅切换 */
    .dark .split > span {
        color: rgba(255, 255, 255, 0.8);
    }

    .light .split > span {
        color: rgba(0, 0, 0, 0.8);
    }

    .main :global(.input) {
        margin-top: 40px;
        margin-bottom: 20px;
        width: 360px;
    }

    .main {
        text-align: center;
    }

    .icon {
        margin-right: 10px;
    }

    .icon > svg {
        width: 28px;
        height: 28px;
        transition: color 0.2s;
    }

    /*  */
    .light svg {
        color: rgba(31, 166, 97, 1);
    }

    .dark svg {
        color: rgba(99, 226, 183, 1);
    }

    .main :global(.button) {
        margin-top: 20px;
        margin-bottom: 20px;
        width: 400px;
    }

    .error-icon {
        margin-top: 20px;
    }

    .error-icon > svg {
        color: #ef5350;
        width: 100px;
    }

    h2 {
        margin: 0;
        font-family: var(--font-sans-serif);
        letter-spacing: 2px;
    }

    .dark h2 {
        color: rgba(255, 255, 255, 0.8);
    }

    .light h2 {
        color: rgba(0, 0, 0, 0.8);
    }

    h3 {
        margin: 0;
        font-family: var(--font-sans-serif);
        letter-spacing: 1px;
        font-weight: lighter;
    }

    .dark h3 {
        color: rgba(255, 255, 255, 0.8);
    }

    .light h3 {
        color: rgba(0, 0, 0, 0.8);
    }
</style>
