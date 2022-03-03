export default {
    mainPage: {
        menu: {
            button: "添加新连接",
            listButton: {
                edit: "编辑",
                delete: "删除",
                connect: "连接",
                disconnect: "断开"
            },
            listItemName: "数据库",
            status: "使用中",
            dialog: {
                title: {
                    edit: "编辑连接",
                    add: "添加连接"
                },
                form: {
                    name: " 名称",
                    host: "连接地址",
                    port: "端口",
                    username: "用户名",
                    password: "密码",
                    submit: "提交",
                    test: "测试",
                    cancel: "取消"
                },
                AddSuccessMsg: "添加成功",
                EditSuccessMsg: "编辑成功",
                TestSuccessMsg: "连接成功",
                ErrorMsg: "操作失败",
                TestErrMsg: "连接测试失败"
            }
        },
        content: {
            tabBar: {
                info: "信息",
                data: "数据"
            },
            baseInfo: {
                lookMore: "查看更多",
                newWindowTitle: "配置详情"
            },
            dataPage: {
                notConnectText: "没有连接",
                searchPlaceHolder: "请输入查询表达式(*a*)",
                databaseList: {
                    title: "数据库",
                    emptyTreeText: "no data"
                },
                actionBar: {
                    add: "增加",
                    rename: "重命名",
                    ttl: 'TTL',
                    del: "删除",
                    refresh: "刷新"
                },
                tableInfo: {
                    title: "Key基础信息",
                    tHeader: ["key", "type", "size", "ttl(ms)"]
                },
                notChooseKey: "请选择key",
                dialog: {
                    title: "新key",
                    form: {
                        type: "类型",
                        key: "key",
                        field: "字段",
                        value: "值",
                        member: "member",
                        timeType: "时间类型",
                        timeTypeOption: {
                            s: "秒",
                            ms: "毫秒"
                        },
                        expire: "过期时间",
                        expirePlaceHolder: "默认为永不过期",
                        button: {
                            submit:"提交",
                            cancel:"取消"
                        }
                    },
                    ruleMessage: {
                        type:"类型不能为空",
                        key:"键不能为空",
                        value:"值不能为空",
                        member:"member不能为空",
                        field:"字段不能为空",
                    },
                },
                renameDialog: {
                    title: "修改记录"
                }
            }
        }
    }
}