export default {
    mainPage: {
        menu: {
            button: "New Connection",
            listButton: {
                edit: "edit",
                delete: "del",
                connect: "conn",
                disconnect: "disc"
            },
            listItemName: "database",
            status: "using",
            dialog: {
                title: {
                    edit: "Edit Connection",
                    add: "Add Connection"
                },
                form: {
                    name: "Name",
                    host: "Host",
                    port: "Port",
                    username: "Username",
                    password: "Password",
                    submit: "submit",
                    test: "test",
                    cancel: "cancel"
                },
                AddSuccessMsg: "Add Successfully",
                EditSuccessMsg: "EditSuccessfully",
                TestSuccessMsg: "Test Successfully",
                ErrorMsg: "Action Fail",
                TestErrMsg: "Connection Test Fail",
            },
            deleteAsk: {
                title: "⚠️ Warning",
                content: "are you sure to remove this connection?"}
        },
        content: {
            tabBar: {
                info: "info",
                data: "data"
            },
            baseInfo: {
                lookMore: "lookMore",
                newWindowTitle: "Configure Detail"
            },
            dataPage: {
                notConnectText: "No have any connection",
                searchPlaceHolder: "query expression(*a*)",
                loadAllKey: "All",
                databaseList: {
                    title: "database",
                    emptyTreeText: "no data"
                },
                actionBar: {
                    add: "add",
                    rename: "rename",
                    ttl: 'TTL',
                    del: "delete",
                    refresh: "refresh"
                },
                tableInfo: {
                    title: "Key BaseInfo",
                    tHeader: ["key", "type", "size", "ttl(ms)"]
                },
                notChooseKey: "Please choose a key",
                dialog: {
                    title: "New Key",
                    form: {
                        type: "type",
                        key: "key",
                        field: "field",
                        value: "value",
                        member: "member",
                        timeType: "timeType",
                        timeTypeOption: {
                            s: "second",
                            ms: "millisecond"
                        },
                        expire: "expire",
                        expirePlaceHolder: "not expired forever is default",
                        button: {
                            submit: "submit",
                            cancel: "cancel"
                        }
                    },
                    ruleMessage: {
                        type: "type is required",
                        key: "key is required",
                        value: "value is required",
                        member: "member is required",
                        field: "field is required",
                    },
                },
                renameDialog: {
                    title: "Rename Record",
                    form: {
                        key: "key",
                        newKey: "new key",
                        submit: "Submit",
                        cancel: "Cancel"
                    },
                    formRuleMsg: {
                        key: "Old Key is Required",
                        newKey: "New Key is Required",
                    }
                },
                ttl: {
                    title: "Set expire time",
                    form: {
                        key: "Key",
                        timeType: "Type",
                        timeTypeOption: {
                            s: "Second",
                            ms: "Millisecond"
                        },
                        expire: "expire",
                        expirePlaceHolder: "-1 cancel expiration",
                        submit: "Submit",
                        cancel: "Cancel"
                    },
                    formRuleMsg: {
                        key: "key is required",
                        timeType: "timeType is required",
                        expire: "expire is required"
                    }
                },
                deleteKeyConfirmMsg: "Are you sure to delete this key?",
                string: {
                    title: "Query Result",
                    format: "Format",
                    formatPlaceholder: "Format Type"
                },
                hash: {
                    title: "Query Result",
                    queryPlaceHolder: "query expression",
                    q: "query",
                    r: "reset",
                    a: "add",
                    tableHeader: ["field", "value", "action"],
                    tableAction: ["modify", "delete", "detail"],
                    pager: {
                        size: "page size",
                        next: "next",
                        end: "ended",
                        reset: "reset flag"
                    },
                    dialog: {
                        modifyTitle: "Modify",
                        addTitle: "Add",
                        button: ["submit", "cancel"],
                        form: {
                            field: "Field",
                            value: "Value"
                        },
                        ruleMsg: {
                            field: "field is required",
                            value: "value is required"
                        }
                    },
                },
                list: {
                    title: "Query Result",
                    button: ["add", "cut"],
                    tableHeader: ["data", "action"],
                    tableAction: ["modify", "delete", "detail"],
                    add:{

                    }
                }
            },
            pubSub: {
                add: "New Subscribe",
                refresh: "Refresh List",
                publish: "Publish",
                subList: "Subscribe List",
                emptyResult: "Result Empty",
                subResult: "Subscribe Result",
                filterPlaceholder: "filter condition",
                button: {
                    submit: "Submit",
                    cancel: "Cancel"
                },
                addDialog: {
                    title: "Add Subscribe",
                    form: {
                        name: "channel name",
                    },
                    formRuleMsg: {
                        name: "channel name is required"
                    }
                },
                publishDialog: {
                    title: "Publish",
                    form: {
                        channel: "Channel",
                        data: "Data"
                    },
                    formRuleMsg: {
                        channel: "channel name is required",
                        data: "data is required"
                    }
                }
            }
        }
    },
    setPage: {
        autoRefresh: "Auto refresh",
        autoRefreshTime: "Refresh time",
        autoRefreshTimePlaceholder: "three seconds is default",
        pubSub: "Pub/Sub",
        lang: "Language",
        save: "Save",
        cancel: "Cancel"
    }
}