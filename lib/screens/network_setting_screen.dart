import 'package:flutter/material.dart';
import 'package:jenny/configs/network_api_host.dart';
import 'package:jenny/configs/network_cdn_host.dart';
import 'package:jenny/screens/init_screen.dart';

import 'components/right_click_pop.dart';

class NetworkSettingScreen extends StatelessWidget {
  const NetworkSettingScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("网络设置"),
        actions: [
          IconButton(
            onPressed: () {
              Navigator.of(context).pushReplacement(
                MaterialPageRoute(builder: (BuildContext context) {
                  return const InitScreen();
                }),
              );
            },
            icon: const Icon(Icons.save),
          ),
        ],
      ),
      body: ListView(
        children: [
          apiHostSetting(),
          cdnHostSetting(),
        ],
      ),
    );
  }
}
