import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/configs/login.dart';
import 'package:jenny/screens/about_screen.dart';
import 'package:jenny/screens/comments_screen.dart';
import 'package:jenny/screens/components/avatar.dart';
import 'package:jenny/screens/pro_screen.dart';
import 'package:jenny/screens/settings_scree.dart';
import 'package:jenny/screens/view_log_screen.dart';

import '../configs/is_pro.dart';
import 'components/badge.dart';
import 'downloads_screen.dart';
import 'favorites_screen.dart';

class UserScreen extends StatefulWidget {
  const UserScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _UserScreenState();
}

class _UserScreenState extends State<UserScreen>
    with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  @override
  void initState() {
    loginEvent.subscribe(_setState);
    proEvent.subscribe(_setState);
    super.initState();
  }

  @override
  void dispose() {
    loginEvent.unsubscribe(_setState);
    proEvent.unsubscribe(_setState);
    super.dispose();
  }

  void _setState(_) {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return Scaffold(
      appBar: AppBar(title: const Text("个人中心"), actions: [
        IconButton(
          onPressed: () {
            Navigator.of(context)
                .push(MaterialPageRoute(builder: (BuildContext context) {
              return const ProScreen();
            }));
          },
          icon: Icon(
            isPro ? Icons.offline_bolt : Icons.offline_bolt_outlined,
          ),
        ),
        _buildSettingsIcon(),
        _buildAboutIcon(),
      ]),
      body: SafeArea(
        child: ListView(
          children: [
            _buildCard(),
            const Divider(),
            _buildFavorites(),
            const Divider(),
            _buildViewLog(),
            const Divider(),
            _buildDownloads(),
            const Divider(),
            _buildComments(),
            const Divider(),
            // _buildFdT(),
            // const Divider(),
            // _buildSettingsT(),
            // const Divider(),
            // _buildAboutT(),
            // const Divider(),
            Container(height: 30),
          ],
        ),
      ),
    );
  }

  Widget _buildCard() {
    late Widget child;
    switch (loginStatus) {
      case LoginStatus.notSet:
        child = _buildLoginButton("登录 / 注册");
        break;
      case LoginStatus.logging:
        child = _buildLoginLoading();
        break;
      case LoginStatus.loginSuccess:
        child = _buildSelfInfoCard();
        break;
      case LoginStatus.loginField:
        child = _buildLoginButton("登录失败/点击重试");
        break;
    }
    return Container(
      height: 200,
      color: Theme.of(context).brightness == Brightness.light
          ? Colors.grey.shade200
          : Colors.grey.shade800,
      child: Center(
        child: child,
      ),
    );
  }

  Widget _buildLoginButton(String title) {
    return MaterialButton(
      onPressed: () async {
        await loginDialog(context);
      },
      child: Container(
        padding: const EdgeInsets.only(left: 15, right: 15, top: 8, bottom: 8),
        decoration: BoxDecoration(
          color: Colors.orange.shade700,
          border: Border.all(
            color: Colors.black,
            style: BorderStyle.solid,
            width: .5,
          ),
          borderRadius: const BorderRadius.all(Radius.circular(4)),
        ),
        child: Text(
          title,
          style: const TextStyle(
            fontSize: 16,
            color: Colors.white,
            fontWeight: FontWeight.normal,
          ),
        ),
      ),
    );
  }

  Widget _buildLoginLoading() {
    return LayoutBuilder(
      builder: (BuildContext context, BoxConstraints constraints) {
        final size = constraints.maxWidth < constraints.maxHeight
            ? constraints.maxWidth
            : constraints.maxHeight;
        return Icon(Icons.refresh,
            size: size * .5, color: Colors.white.withOpacity(.5));
      },
    );
  }

  Widget _buildSelfInfoCard() {
    return Column(
      children: [
        Expanded(child: Container()),
        Center(
          child: Avatar(selfInfo.photo),
        ),
        Container(height: 10),
        Center(
          child: Text(
            selfInfo.username,
            style: TextStyle(
              color: Theme.of(context).brightness == Brightness.light
                  ? Colors.black87
                  : Colors.white,
            ),
          ),
        ),
        Expanded(child: Container()),
      ],
    );
  }

  Widget _buildFavorites() {
    return ListTile(
      onTap: () async {
        if (LoginStatus.loginSuccess == loginStatus) {
          Navigator.of(context).push(MaterialPageRoute(
            builder: (BuildContext context) {
              return const FavoritesScreen();
            },
          ));
        } else {
          defaultToast(context, "登录之后才能使用收藏夹喔");
        }
      },
      title: const Text("收藏夹"),
    );
  }

  Widget _buildViewLog() {
    return ListTile(
      onTap: () async {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const ViewLogScreen();
          },
        ));
      },
      title: const Text("浏览记录"),
    );
  }

  Widget _buildDownloads() {
    return ListTile(
      onTap: () async {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const DownloadsScreen();
          },
        ));
      },
      title: const Text("下载列表"),
    );
  }

  Widget _buildComments() {
    return ListTile(
      onTap: () async {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const CommentsScreen();
          },
        ));
      },
      title: const Text("讨论区"),
    );
  }

  Widget _buildSettingsIcon() {
    return IconButton(
      onPressed: () async {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const SettingsScreen();
          },
        ));
      },
      icon: const Icon(Icons.settings),
    );
  }

  Widget _buildAboutIcon() {
    return IconButton(
      onPressed: () async {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const AboutScreen();
          },
        ));
      },
      icon: const VersionBadged(
        child: Padding(
          padding: EdgeInsets.all(1),
          child: Icon(Icons.info_outlined),
        ),
      ),
    );
  }

  Widget _buildFdT() {
    return ListTile(
      title: const Text("发电"),
      onTap: () {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const ProScreen();
          },
        ));
      },
    );
  }

  Widget _buildSettingsT() {
    return ListTile(
      title: const Text("设置"),
      onTap: () {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const SettingsScreen();
          },
        ));
      },
    );
  }

  Widget _buildAboutT() {
    return ListTile(
      title: const VersionBadged(
        child: Text("关于"),
      ),
      onTap: () {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const AboutScreen();
          },
        ));
      },
    );
  }
}
