import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/components/comic_list.dart';

import 'components/comic_comments_list.dart';
import 'components/right_click_pop.dart';

class CommentsScreen extends StatefulWidget {
  const CommentsScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _CommentsScreenState();
}

class _CommentsScreenState extends State<CommentsScreen> {

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("讨论区"),
      ),
      body: ListView(
        children: const [
          ComicCommentsList(mode: "manhua", aid: null, gotoComic: true),
        ],
      ),
    );
  }
}
