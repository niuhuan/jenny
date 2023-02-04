/// 代理设置

import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

late String _currentProxy;

Future<String?> initProxy() async {
  _currentProxy = await methods.getProxy();
  return null;
}

String currentProxyName() {
  return _currentProxy == "" ? "未设置" : _currentProxy;
}

Future<dynamic> inputProxy(BuildContext context) async {
  String? input = await displayTextInputDialog(
    context,
    src: _currentProxy,
    title: '代理服务器',
    hint: '请输入代理服务器',
    desc: " ( 例如 socks5://127.0.0.1:1080/ ) ",
  );
  if (input != null) {
    await methods.setProxy(input);
    _currentProxy = input;
  }
}

Widget proxySetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("代理服务器"),
        subtitle: Text(currentProxyName()),
        onTap: () async {
          await inputProxy(context);
          setState(() {});
        },
      );
    },
  );
}
