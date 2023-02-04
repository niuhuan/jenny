import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/configs/is_pro.dart';

enum LoginStatus {
  notSet,
  logging,
  loginField,
  loginSuccess,
}

late SelfInfo _selfInfo;
LoginStatus _status = LoginStatus.notSet;
String _loginMessage = "";
final Event _event = Event();

SelfInfo get selfInfo => _selfInfo;

Event get loginEvent => _event;

LoginStatus get loginStatus => _status;

String get loginMessage => _loginMessage;

set _loginState(LoginStatus value) {
  _status = value;
  _event.broadcast();
}

Future initLogin() async {
  try {
    _loginState = LoginStatus.logging;
    final preLogin = await methods.preLogin();
    _loginMessage = preLogin.message ?? "";
    if (!preLogin.preSet) {
      _loginState = LoginStatus.notSet;
    } else if (preLogin.preLogin) {
      _selfInfo = preLogin.selfInfo!;
      _loginState = LoginStatus.loginSuccess;
    } else {
      _loginState = LoginStatus.loginField;
    }
  } catch (e, st) {
    print("$e\n$st");
    _loginState = LoginStatus.loginField;
  } finally {
    reloadIsPro();
  }
}

Future login(String username, String password) async {
  try {
    _loginState = LoginStatus.logging;
    final selfInfo = await methods.login(username, password);
    _selfInfo = selfInfo;
    _loginState = LoginStatus.loginSuccess;
  } catch (e, st) {
    print("$e\n$st");
    _loginState = LoginStatus.loginField;
    _loginMessage = "$e";
  }
}

Future loginDialog(BuildContext context) async {
  await showDialog(
    context: context,
    builder: (BuildContext context) {
      return Container(
        width: 30,
        height: 30,
        color: Colors.black.withOpacity(.1),
        child: Center(
          child: _LoginDialog(),
        ),
      );
    },
  );
}

class _LoginDialog extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _LoginDialogState();
}

class _LoginDialogState extends State<_LoginDialog> {
  var _username = "";
  var _password = "";

  @override
  void initState() {
    Future.delayed(Duration.zero, () async {
      final username = await methods.loadUsername();
      final password = await methods.loadPassword();
      setState(() {
        _username = username;
        _password = password;
      });
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 250,
      width: MediaQuery.of(context).size.width - 90,
      margin: const EdgeInsets.all(30),
      decoration: BoxDecoration(
        borderRadius: const BorderRadius.all(Radius.circular(10)),
        color: Theme.of(context).scaffoldBackgroundColor,
      ),
      child: Material(
        color: Colors.transparent,
        child: ListView(
          children: [
            Row(
              children: [
                IconButton(
                  onPressed: () {
                    Navigator.of(context).pop();
                  },
                  icon: const Icon(Icons.arrow_back),
                ),
                Expanded(child: Container()),
              ],
            ),
            ListTile(
              title: Text("账号"),
              subtitle: Text(_username == "" ? "未设置" : _username),
              onTap: () async {
                String? input = await displayTextInputDialog(
                  context,
                  src: _username,
                  title: '账号',
                  hint: '请输入账号',
                );
                if (input != null) {
                  setState(() {
                    _username = input;
                  });
                }
              },
            ),
            ListTile(
              title: const Text("密码"),
              subtitle: Text(_password == "" ? "未设置" : '\u2022' * 10),
              onTap: () async {
                String? input = await displayTextInputDialog(
                  context,
                  src: _password,
                  title: '密码',
                  hint: '请输入密码',
                  isPasswd: true,
                );
                if (input != null) {
                  setState(() {
                    _password = input;
                  });
                }
              },
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                Container(
                  margin: const EdgeInsets.all(10),
                  child: MaterialButton(
                    color: Colors.orange.shade700,
                    onPressed: () async {
                      String? fen = await chooseMapDialog(
                        context,
                        values: {
                          "注册分流1": "https://18comic.vip/signup",
                          "注册分流2": "https://18comic.org/signup",
                          "注册分流3": "https://jmcomic.me/signup",
                          "注册分流4": "https://jmcomic1.me/signup",
                          "注册分流5": "https://jmcomic.asia/signup",
                          "注册分流6": "https://jmcomic.city/signup",
                        },
                        title: "选择注册分流",
                      );
                      if (fen != null) {
                        openUrl(fen);
                      }
                    },
                    child: Container(
                      padding: const EdgeInsets.all(10),
                      child: const Text(
                        "注册",
                        style: TextStyle(color: Colors.white),
                      ),
                    ),
                  ),
                ),
                Container(
                  margin: const EdgeInsets.all(10),
                  child: MaterialButton(
                    color: Colors.orange.shade700,
                    onPressed: () async {
                      Navigator.of(context).pop();
                      await login(_username, _password);
                      await reloadIsPro();
                    },
                    child: Container(
                      padding: const EdgeInsets.all(10),
                      child: const Text(
                        "保存",
                        style: TextStyle(color: Colors.white),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
