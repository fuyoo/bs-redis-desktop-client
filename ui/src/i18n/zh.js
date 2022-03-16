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
            },
            deleteAsk: {
                title: "⚠️ 警告",
                content: "你确定要删除这个连接吗？"}
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
                loadAllKey: "所有",
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
                            submit: "提交",
                            cancel: "取消"
                        }
                    },
                    ruleMessage: {
                        type: "类型不能为空",
                        key: "键不能为空",
                        value: "值不能为空",
                        member: "member不能为空",
                        field: "字段不能为空",
                    },
                },
                renameDialog: {
                    title: "修改记录",
                    form: {
                        key: "key",
                        newKey: "新key",
                        submit: "提交",
                        cancel: "取消"
                    },
                    formRuleMsg: {
                        key: "旧key不能为空",
                        newKey: "新Key不能为空",
                    }
                },
                ttl: {
                    title: "设置过期时间",
                    form: {
                        key: "Key",
                        timeType: "时间类型",
                        timeTypeOption: {
                            s: "秒",
                            ms: "毫秒"
                        },
                        expire: "时间",
                        expirePlaceHolder: "-1 取消过期",
                        submit: "提交",
                        cancel: "取消"
                    },
                    formRuleMsg: {
                        key: "key不能为空",
                        timeType: "时间类型不能为空",
                        expire: "过期时间不能为空"
                    }
                },
                deleteKeyConfirmMsg: "确定要删除这个key吗?",
                string: {
                    title: "查询结果",
                    format: "格式化",
                    formatPlaceholder: "格式化类型"
                },
                hash: {
                    title: "查询结果",
                    queryPlaceHolder:"请输入查询表达式",
                    q: "查询",
                    r: "重置",
                    a: "添加",
                    tableHeader: ["字段", "值", "操作"],
                    tableAction: ["修改", "删除", "详情"],
                    pager: {
                        size: "每一页条数",
                        next: "下一页",
                        end: "已经是最后一页了",
                        reset: "重置游标"
                    },
                    dialog: {
                        modifyTitle: "修改",
                        addTitle: "添加",
                        button: ["提交", "取消"],
                        form: {
                            field: "字段",
                            value: "值"
                        },
                        ruleMsg: {
                            field: "字段不能为空",
                            value: "值不能为空"
                        }
                    },
                },
                list: {
                    title: "查询结果",
                    button: ["添加", "裁剪"],
                    tableHeader: ["数据", "裁剪"],
                    tableAction: ["修改", "删除", "详情"],
                    add:{

                    }
                }
            },
            pubSub: {
                add: "添加新订阅",
                refresh: "刷新列表",
                publish: "发布消息",
                subList: "订阅列表",
                emptyResult: "没有数据",
                subResult: "订阅结果",
                filterPlaceholder: "过滤条件",
                button: {
                    submit: "提交",
                    cancel: "取消"
                },
                addDialog: {
                    title: "添加新订阅",
                    form: {
                        name: "通道名称",
                    },
                    formRuleMsg: {
                        name: "通道名称不能为空"
                    }
                },
                publishDialog: {
                    title: "发布消息",
                    form: {
                        channel: "通道名称",
                        data: "数据"
                    },
                    formRuleMsg: {
                        channel: "通道名称不能为空",
                        data: "数据不能为空"
                    }
                }
            }

        }
    },
    setPage: {
        autoRefresh: "自动刷新",
        autoRefreshTime: "刷新时间",
        autoRefreshTimePlaceholder: "默认3秒",
        pubSub: "Pub/Sub",
        lang: "语言",
        save: "保存",
        cancel: "取消"
    }
}