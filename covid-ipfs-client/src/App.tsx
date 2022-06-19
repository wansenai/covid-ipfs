import React, { FC } from 'react';
import { Button, message, Result, notification} from 'antd';
import { SmileOutlined } from '@ant-design/icons';
import './App.css';
import {
  ProCard,
  ProForm,
  ProFormCheckbox,
  ProFormDatePicker,
  ProFormDateRangePicker,
  ProFormSelect,
  ProFormText,
  ProFormTextArea,
  ProFormRadio,
  ProFormDigit,
  StepsForm,
} from '@ant-design/pro-components';
import axios from 'axios'

const waitTime = (time: number = 100) => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(true);
    }, time);
  });
};

type NotificationType = 'success' | 'info' | 'warning' | 'error';
const openNotificationWithIcon = (type: NotificationType, content: string) => {
  notification[type]({
    message: '消息通知',
    description: content
  });
};

const App: FC = () => {
  return (
    <ProCard>
      <StepsForm<{
        name: string;
      }>
        onFinish={async (values) => {
          console.log(values);

          // axios request api
          axios({
            url:'http://101.43.52.162:8082/nucleic/save',
            headers: {
              'Content-Type': 'application/json'
            },
            data: values,
            method:'post'
            }).then(
              res => openNotificationWithIcon('success', '核酸结果hash: ' + res.data.data)
            ).catch(
              err => openNotificationWithIcon('error', '核酸结果保存失败')
            )
        }}
        formProps={{
          validateMessages: {
            required: '此项为必填项',
          },
        }}
        submitter={{
          render: (props) => {
            if (props.step === 0) {
              return (
                <Button type="primary" onClick={() => props.onSubmit?.()}>
                  下一步 {'>'}
                </Button>
              );
            }

            if (props.step === 1) {
              return [
                <Button key="pre" onClick={() => props.onPre?.()}>
                  返回上一步
                </Button>,
                <Button type="primary" key="goToTree" onClick={() => props.onSubmit?.()}>
                  下一步 {'>'}
                </Button>,
              ];
            }

            return [
              <Button key="gotoTwo" onClick={() => props.onPre?.()}>
                {'<'} 返回上一步
              </Button>,
              <Button type="primary" key="goToTree" onClick={() => props.onSubmit?.()}>
                保存结果到IPFS
              </Button>,
            ];
          },
        }}
      >
        <StepsForm.StepForm<{
          name: string;
        }>
          name="base"
          title="创建用户信息"
          onFinish={async ({ name }) => {
            console.log(name);
            // await waitTime(2000);
            return true;
          }}
        >
          <ProFormText
            id="name"
            name="name"
            label="用户名"
            width="md"
            tooltip="用户的姓名"
            placeholder="请输入姓名"
            rules={[{ required: true }]}
          />
          <ProFormRadio.Group id="sex" name="sex" label="性别"
              options={[
                {
                  label: '男',
                  value: 1,
                },
                {
                  label: '女',
                  value: 0,
                }
              ]} 
              rules={[{ required: true }]}
          />
          <ProFormDigit id="phone" name="phone" label="电话"  width="md" rules={[{ required: true }]}/>
          <ProFormTextArea id="address" name="address" label="住址" width="md" placeholder="请输入居住地, xxx省xxx市xxx街道xxx路" rules={[{ required: true }]}/>
          <ProFormDatePicker id="date" name="date" label="日期" rules={[{ required: true }]}/>
        </StepsForm.StepForm>
        <StepsForm.StepForm<{
          checkbox: string;
        }>
          name="checkbox"
          title="填写核酸信息"
        >
          <ProFormCheckbox.Group
            name="checkbox"
            label="核酸机构名称"
            width="lg"
            options={['上海市第十人民医院', '上海中检医学检验所（气膜）', '上海锦测医学检验所（气膜）', 
            '上海华测艾普医学检验所', '河南省卫健委2舱', '湖州市城市核酸检测基地', '浙江医疗队迪安气膜实验室']}
          />
          <ProFormSelect
            id = "result"
            label="核酸类型"
            name="result"
            rules={[
              {
                required: true,
              },
            ]}
            options={[
              { value: 'NEGATIVE', label: '阴性' },
              { value: 'POSITIVE', label: '阳性' },
            ]}
          />
          <ProFormTextArea name="remake" label="备注" width="xl" placeholder="检验结果具体信息"/>
        </StepsForm.StepForm>
        <StepsForm.StepForm name="time" title="发布核酸结果">
        <Result
          icon={<SmileOutlined />}
          title="核酸信息收集完成，确认无误请保存"
          />
        </StepsForm.StepForm>
      </StepsForm>
    </ProCard>
  );
};

export default App;