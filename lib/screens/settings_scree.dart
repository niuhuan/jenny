import 'dart:io';

import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/downloads_exports_screen2.dart';

import '../basic/commons.dart';
import '../basic/web_dav_sync.dart';
import '../configs/android_display_mode.dart';
import '../configs/download_and_export_to.dart';
import '../configs/export_rename.dart';
import '../configs/no_animation.dart';
import '../configs/proxy.dart';
import '../configs/theme.dart';
import '../configs/using_right_click_pop.dart';
import '../configs/volume_key_control.dart';
import '../configs/web_dav_password.dart';
import '../configs/web_dav_sync_switch.dart';
import '../configs/web_dav_url.dart';
import '../configs/web_dav_username.dart';
import 'components/right_click_pop.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() {
    return _SettingsState();
  }
}

class _SettingsState extends State<SettingsScreen> {
  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("设置"),
      ),
      body: ListView(
        children: [
          const Divider(),
          proxySetting(),
          const Divider(),
          themeSetting(context),
          const Divider(),
          androidDisplayModeSetting(),
          const Divider(),
          usingRightClickPopSetting(),
          const Divider(),
          volumeKeyControlSetting(),
          noAnimationSetting(),
          const Divider(),
          exportRenameSetting(),
          downloadAndExportToSetting(),
          const Divider(),
          webDavSyncSwitchSetting(),
          webDavUrlSetting(),
          webDavUserNameSetting(),
          webDavPasswordSetting(),
          webDavSyncClick(context),
          const Divider(),
          const Divider(),
          ListTile(
            onTap: () async {
              if (await confirmDialog(
                  context, "清除账号信息", "您确定要清除账号信息并退出APP吗?")) {
                await methods.logout();
                exit(0);
              }
            },
            title: const Text("清除账号信息"),
          ),
          const Divider(),
          ListTile(
            onTap: () {
              Navigator.of(context).push(MaterialPageRoute(
                  builder: (c) => const DownloadsExportScreen2()));
            },
            title: const Text("导出下载到目录(即使没有下载完)"),
          ),
          const Divider(),
        ],
      ),
    );
  }
}
