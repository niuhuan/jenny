import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/screens/app_screen.dart';
import 'package:jenny/screens/components/content_loading.dart';

import '../configs/is_pro.dart';
import '../configs/login.dart';
import '../configs/network_api_host.dart';
import '../configs/network_cdn_host.dart';

const firstLoginScreen = FirstLoginScreen();

class FirstLoginScreen extends StatefulWidget {
  const FirstLoginScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _FirstLoginScreenState();
}

class _FirstLoginScreenState extends State<FirstLoginScreen> {
  bool _logging = false;
  String _username = "";
  String _password = "";

  Widget _usernameField() {
    return ListTile(
      title: const Text("账号"),
      subtitle: Text(_username),
      onTap: () async {
        final input = await displayTextInputDialog(
          context,
          hint: "请输入账号",
          title: "账号",
          src: _username,
        );
        if (input != null) {
          setState(() {
            _username = input;
          });
        }
      },
    );
  }

  Widget _passwordField() {
    return ListTile(
      title: const Text("密码"),
      subtitle: Text(_password.isEmpty ? "" : '********'),
      onTap: () async {
        final input = await displayTextInputDialog(
          context,
          hint: "请输入密码",
          title: "密码",
          isPasswd: true,
          src: _password,
        );
        if (input != null) {
          setState(() {
            _password = input;
          });
        }
      },
    );
  }

  late final _saveButton = IconButton(
    onPressed: () async {
      setState(() {
        _logging = true;
      });
      await login(_username, _password);
      await reloadIsPro();
      if (loginStatus != LoginStatus.loginSuccess) {
        defaultToast(context, loginMessage);
        setState(() {
          _logging = false;
        });
      } else {
        Navigator.pushReplacement(context, MaterialPageRoute(
          builder: (BuildContext context) {
            return const AppScreen();
          },
        ));
      }
    },
    icon: const Icon(Icons.save),
  );

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("登录"),
        actions: _logging
            ? []
            : [
                _saveButton,
              ],
      ),
      body: ListView(
        children: _logging
            ? [
                const ContentLoading(),
              ]
            : [
                _usernameField(),
                _passwordField(),
                apiHostSetting(),
                cdnHostSetting(),
              ],
      ),
    );
  }
}
