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
            }
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
                    title: "Rename Record"
                }
            }
        }
    }
}