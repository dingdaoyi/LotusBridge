## 自定义表单json格式
```javascript
[

    {
        name: 'name',
        eltype: 'input',
        label: '姓名',
        placeholder: '输入框姓名'
    },
    {
        name: 'date',
        eltype: 'date',
        label: '日期',
        placeholder: '日期',
    },
    {
        name: 'switch',
        eltype: 'switch',
        label: '状态',
        placeholder: '状态',
    },
    {
        name: 'radio',
        eltype: 'radio',
        label: '性别',
        placeholder: '性别',
        list: [
            {
                label: '男',
                value: '1'
            },
            {
                label: '女',
                value: '2'
            }
        ]
    },
    {
        name: 'select',
        eltype: 'select',
        label: '选择框',
        placeholder: '输入框',
        url: '/district/v1/getchildren',
        props: {
            label: 'fullname',
            value: 'id'
        }
    }
]
```