import 'dart:io';

import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';

import '../basic/commons.dart';
import '../configs/export_rename.dart';
import '../configs/is_pro.dart';
import 'components/content_loading.dart';
import 'components/right_click_pop.dart';

class ExportAb {
  final int id;
  final String name;

  ExportAb(this.id, this.name);
}

class DownloadsExportingScreen extends StatefulWidget {
  final List<int> idList;

  const DownloadsExportingScreen({Key? key, required this.idList})
      : super(key: key);

  @override
  State<StatefulWidget> createState() => _DownloadsExportingScreenState();
}

class _DownloadsExportingScreenState extends State<DownloadsExportingScreen> {
  bool exporting = false;
  bool exported = false;
  bool exportFail = false;
  dynamic e;
  String exportMessage = "正在导出";

  @override
  void initState() {
    //registerEvent(_onMessageChange, "EXPORT");
    super.initState();
  }

  @override
  void dispose() {
    //unregisterEvent(_onMessageChange);
    super.dispose();
  }

  void _onMessageChange(event) {
    setState(() {
      exportMessage = event;
    });
  }

  Widget _body() {
    if (exporting) {
      return ContentLoading(label: exportMessage);
    }
    if (exportFail) {
      return Center(child: Text("导出失败\n$e\n($exportMessage)"));
    }
    if (exported) {
      return const Center(child: Text("导出成功"));
    }
    return ListView(
      children: [
        // Container(height: 20),
        // MaterialButton(
        //   onPressed: _exportPkz,
        //   child: const Text("导出PKZ"),
        // ),
        Container(height: 20),
        MaterialButton(
          onPressed: _exportPkis,
          child: Text(
            "分别导出JMI" + (!isPro ? "\n(发电后使用)" : ""),
            style: TextStyle(
              color: !isPro ? Colors.grey : null,
            ),
            textAlign: TextAlign.center,
          ),
        ),
        Container(height: 20),
        MaterialButton(
          onPressed: _exportZips,
          child: Text(
            "分别导出JM.ZIP" + (!isPro ? "\n(发电后使用)" : ""),
            style: TextStyle(
              color: !isPro ? Colors.grey : null,
            ),
            textAlign: TextAlign.center,
          ),
        ),
        Container(height: 20),
      ],
    );
  }

  _exportPkis() async {
    if (!isPro) {
      defaultToast(context, "请先发电鸭");
      return;
    }
    late String? path;
    try {
      path = Platform.isIOS
          ? await methods.iosGetDocumentDir()
          : await chooseFolder(context);
    } catch (e) {
      defaultToast(context, "$e");
      return;
    }
    print("path $path");
    if (path != null) {
      try {
        setState(() {
          exporting = true;
        });
        for (var value in widget.idList) {
          var ab = await methods.downloadById(value);
          setState(() {
            exportMessage = "正在导出 : " + (ab?.album?.name ?? "");
          });
          String? rename;
          if (currentExportRename()) {
            rename = await displayTextInputDialog(context, title: "导出重命名", src: ab?.album?.name ?? "");
          }
          await methods.export_jm_jmi_single(
            value,
            path,
            rename,
          );
        }
        exported = true;
      } catch (err) {
        e = err;
        exportFail = true;
      } finally {
        setState(() {
          exporting = false;
        });
      }
    }
  }

  _exportZips() async {
    if (!isPro) {
      defaultToast(context, "请先发电鸭");
      return;
    }
    late String? path;
    try {
      path = Platform.isIOS
          ? await methods.iosGetDocumentDir()
          : await chooseFolder(context);
    } catch (e) {
      defaultToast(context, "$e");
      return;
    }
    print("path $path");
    if (path != null) {
      try {
        setState(() {
          exporting = true;
        });
        for (var value in widget.idList) {
          var ab = await methods.downloadById(value);
          setState(() {
            exportMessage = "正在导出 : " + (ab?.album?.name ?? "");
          });
          String? rename;
          if (currentExportRename()) {
            rename = await displayTextInputDialog(context, title: "导出重命名", src: ab?.album?.name ?? "");
          }
          await methods.export_jm_zip_single(
            value,
            path,
            rename,
          );
        }
        exported = true;
      } catch (err) {
        e = err;
        exportFail = true;
      } finally {
        setState(() {
          exporting = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return rightClickPop(
      child: buildScreen(context),
      context: context,
      canPop: !exporting,
    );
  }

  Widget buildScreen(BuildContext context) {
    return WillPopScope(
      child: Scaffold(
        appBar: AppBar(
          title: const Text("批量导出"),
        ),
        body: _body(),
      ),
      onWillPop: () async {
        if (exporting) {
          defaultToast(context, "导出中, 请稍后");
          return false;
        }
        return true;
      },
    );
  }
}
