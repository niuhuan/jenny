import 'dart:io';

import 'package:clipboard/clipboard.dart';
import 'package:file_picker/file_picker.dart';
import 'package:filesystem_picker/filesystem_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_styled_toast/flutter_styled_toast.dart';
import 'package:jenny/basic/methods.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:url_launcher/url_launcher.dart';
import '../configs/android_version.dart';

const coverShape = RoundedRectangleBorder(
  borderRadius: BorderRadius.all(Radius.circular(4.5)),
);

/// 显示一个toast
void defaultToast(BuildContext context, String title) {
  showToast(
    title,
    context: context,
    position: StyledToastPosition.center,
    animation: StyledToastAnimation.scale,
    reverseAnimation: StyledToastAnimation.fade,
    duration: const Duration(seconds: 4),
    animDuration: const Duration(seconds: 1),
    curve: Curves.elasticOut,
    reverseCurve: Curves.linear,
  );
}

Future<T?> chooseListDialog<T>(BuildContext context,
    {required List<T> values, required String title, String? tips}) async {
  return showDialog<T>(
    context: context,
    builder: (BuildContext context) {
      return SimpleDialog(
        title: Text(title),
        children: [
          ...values.map((e) =>
              SimpleDialogOption(
                onPressed: () {
                  Navigator.of(context).pop(e);
                },
                child: Text('$e'),
              )),
          ...tips != null
              ? [
            Container(
              padding: const EdgeInsets.fromLTRB(15, 5, 15, 15),
              child: Text(tips),
            ),
          ]
              : [],
        ],
      );
    },
  );
}

Future<T?> chooseMapDialog<T>(BuildContext buildContext, {
  required String title,
  required Map<String, T> values,
}) async {
  return await showDialog<T>(
    context: buildContext,
    builder: (BuildContext context) {
      return SimpleDialog(
        title: Text(title),
        children: values.entries
            .map((e) =>
            SimpleDialogOption(
              child: Text(e.key),
              onPressed: () {
                Navigator.of(context).pop(e.value);
              },
            ))
            .toList(),
      );
    },
  );
}

Future saveImageFileToGallery(BuildContext context, String path) async {
  if (Platform.isAndroid) {
    if (androidVersion >= 30) {
      if (!(await Permission.storage.request()).isGranted) {
        throw Exception("申请权限被拒绝");
      }
    } else {
      if (!(await Permission.storage.request()).isGranted) {
        throw Exception("申请权限被拒绝");
      }
    }
  }
  if (Platform.isIOS || Platform.isAndroid) {
    await methods.saveImageFileToGallery(path);
    defaultToast(context, "保存成功");
    return;
  }
  defaultToast(context, "暂不支持该平台");
}

Future<SortBy?> chooseSortBy(BuildContext context) async {
  return await chooseListDialog(
      context, title: "请选择排序方式", values: sorts);
}

/// 将字符串前面加0直至满足len位
String add0(int num, int len) {
  var rsp = "$num";
  while (rsp.length < len) {
    rsp = "0$rsp";
  }
  return rsp;
}

/// 打开web页面
Future<dynamic> openUrl(String url) async {
  if (await canLaunch(url)) {
    await launch(
      url,
      forceSafariVC: false,
    );
  }
}

final _controller = TextEditingController();

Future<String?> displayTextInputDialog(BuildContext context,
    {String? title,
      String src = "",
      String? hint,
      String? desc,
      bool isPasswd = false}) {
  _controller.text = src;
  return showDialog(
    context: context,
    builder: (context) {
      return AlertDialog(
        title: title == null ? null : Text(title),
        content: SingleChildScrollView(
          child: ListBody(
            children: [
              TextField(
                controller: _controller,
                decoration: InputDecoration(hintText: hint),
                obscureText: isPasswd,
                obscuringCharacter: '\u2022',
              ),
              ...(desc == null
                  ? []
                  : [
                Container(
                  padding: EdgeInsets.only(top: 20, bottom: 10),
                  child: Text(
                    desc,
                    style: TextStyle(
                        fontSize: 12,
                        color: Theme
                            .of(context)
                            .textTheme
                            .bodyText1
                            ?.color
                            ?.withOpacity(.5)),
                  ),
                )
              ]),
            ],
          ),
        ),
        actions: <Widget>[
          MaterialButton(
            child: Text('取消'),
            onPressed: () {
              Navigator.of(context).pop();
            },
          ),
          MaterialButton(
            child: Text('确认'),
            onPressed: () {
              Navigator.of(context).pop(_controller.text);
            },
          ),
        ],
      );
    },
  );
}

/// 复制内容到剪切板
void copyToClipBoard(BuildContext context, String string) {
  FlutterClipboard.copy(string);
  defaultToast(context, "已复制到剪切板");
}

/// 显示一个确认框, 用户关闭弹窗以及选择否都会返回false, 仅当用户选择确定时返回true
Future<bool> confirmDialog(BuildContext context, String title,
    String content) async {
  return await showDialog(
      context: context,
      builder: (context) =>
          AlertDialog(
            title: Text(title),
            content: new SingleChildScrollView(
              child: new ListBody(
                children: <Widget>[
                  new Text(content),
                ],
              ),
            ),
            actions: <Widget>[
              new MaterialButton(
                child: new Text('取消'),
                onPressed: () {
                  Navigator.of(context).pop(false);
                },
              ),
              new MaterialButton(
                child: new Text('确定'),
                onPressed: () {
                  Navigator.of(context).pop(true);
                },
              ),
            ],
          )) ??
      false;
}

/// 复制对话框
void confirmCopy(BuildContext context, String content) async {
  if (await confirmDialog(context, "复制", content)) {
    copyToClipBoard(context, content);
  }
}

/// 选择一个文件夹用于保存文件
Future<String?> chooseFolder(BuildContext context) async {
  return FilePicker.platform.getDirectoryPath(
    dialogTitle: "选择一个文件夹, 将文件保存到这里",
  );
}
