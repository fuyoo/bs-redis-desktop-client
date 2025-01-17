// This is just an example,
// so you can safely delete all default props below

export default {
  exit: 'Are you sure exit this app？',
  menu: [
    ['Host', 'redis database host'],
    ['Settings', 'app settings'],
    ['Github', 'github.com/fuyoo/bs-redis-desktop-client'],
  ],
  settings: ['Settings', 'Language', 'Version'],
  actions: ['Ok', 'Cancel', 'Delete', 'Modify', 'Add'],
  home: {
    form: {
      lable: ['Name', 'Host', 'Port', 'Database', 'Username', 'Password', 'Cluster'],
      hint: [
        'Record Name(Required)',
        'Connection Host(Required)',
        'Connection Port(not Required)',
        'Redis Database(not Required)',
        'Redis Auth Username(not Required)',
        'Redis Auth Password(not Required)',
      ],
      rule: ['Host Name is Required', 'Connection host is Required'],
    },
  },
  tabs: {
    tabName: ['Status', 'Data'],
  },
}
