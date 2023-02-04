
import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

late String _currentWebDavPassword;
const _propertyName = "WebDavPassword";

String get currentWebDavPassword => _currentWebDavPassword;

Future<String?> initWebDavPassword() async {
  _currentWebDavPassword = await methods.loadProperty(_propertyName);
  return null;
}

String currentWebDavPasswordName() {
  return _currentWebDavPassword == "" ? "未设置" : _currentWebDavPassword;
}

Future<dynamic> inputWebDavPassword(BuildContext context) async {
  String? input = await displayTextInputDialog(
    context,
    src: _currentWebDavPassword,
    title: 'WebDAV密码',
    hint: '请输入WebDAV密码',
  );
  if (input != null) {
    await methods.saveProperty(_propertyName, input);
    _currentWebDavPassword = input;
  }
}

Widget webDavPasswordSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text("WebDAV密码"),
        subtitle: Text(currentWebDavPasswordName()),
        onTap: () async {
          await inputWebDavPassword(context);
          setState(() {});
        },
      );
    },
  );
}
