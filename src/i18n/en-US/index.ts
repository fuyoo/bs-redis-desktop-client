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
  actions: [
    'Ok',
    'Cancel',
    'Delete',
    'Edit',
    'Add',
    'Insert',
    'Download',
    'Test Connection',
    'Connect',
    'Save',
  ],

  keyForm: {
    label: ['Key Name', 'Data', 'Expire'],
    msg: ['Key name is required', 'Data is required'],
  },
  home: {
    name: 'Home',
    form: {
      label: ['Name', 'Host', 'Port', 'Database', 'Username', 'Password', 'Cluster'],
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
  hostInfo: ['Memory', 'Server', 'Stats', 'Details', 'Key Anylaysis'],
  normal: ['Database', 'Please select a key.'],
  timeFormat: ['d', 'h', 'm', 's', 'mill', 'never'],
  table: ['Data', 'Operate'],
  tips: [
    'Current data type unsupported yet.',
    'Due to the data being larger than {size}, the displayed data is truncated. You can click the button next to it to download the original data',
    'Currently Key {key} not found.',
    'Are sure delete it？',
  ],
  title: ['New {type} Type Key'],
}
