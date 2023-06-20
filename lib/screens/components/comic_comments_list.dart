import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/components/item_builder.dart';

import '../comic_info_screen.dart';
import 'avatar.dart';

class ComicCommentsList extends StatefulWidget {
  final String? mode;
  final int? aid;
  final bool gotoComic;

  const ComicCommentsList({
    required this.mode,
    required this.aid,
    this.gotoComic = false,
    Key? key,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ComicCommentsListState();
}

class _ComicCommentsListState extends State<ComicCommentsList> {
  late Future<CommentPage> _future;
  int _maxPage = 1;
  int _page = 1;

  Future<CommentPage> _loadPage() async {
    final response = await methods.forum(widget.mode, widget.aid, _page);
    if (_page == 1) {
      if (response.total == 0) {
        _maxPage = 1;
      } else {
        _maxPage = (response.total / response.list.length).ceil();
      }
    }
    return response;
  }

  @override
  void initState() {
    _future = _loadPage();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return ItemBuilder(
      future: _future,
      onRefresh: () async {
        setState(() {
          _future = _loadPage();
        });
      },
      successBuilder: (
        BuildContext context,
        AsyncSnapshot<CommentPage> snapshot,
      ) {
        return Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _buildPrePage(),
            ...snapshot.requireData.list.map((e) => _buildComment(
                  context,
                  e,
                  true,
                  widget.mode,
                  widget.gotoComic,
                )),
            _buildNextPage(),
            ...widget.aid != null
                ? [
                    _buildPostComment(
                      context,
                      null,
                      widget.aid!,
                      () {
                        setState(() {
                          _future = _loadPage();
                        });
                      },
                    ),
                  ]
                : [],
          ],
        );
      },
    );
  }

  Widget _buildPrePage() {
    if (_page > 1) {
      return InkWell(
        onTap: () {
          setState(() {
            _page--;
            _future = _loadPage();
          });
        },
        child: Container(
          padding: const EdgeInsets.all(30),
          child: const Center(
            child: Text('上一页'),
          ),
        ),
      );
    }
    return Container();
  }

  Widget _buildNextPage() {
    if (_page < _maxPage) {
      return InkWell(
        onTap: () {
          setState(() {
            _page++;
            _future = _loadPage();
          });
        },
        child: Container(
          padding: const EdgeInsets.all(30),
          child: const Center(
            child: Text('下一页'),
          ),
        ),
      );
    }
    return Container();
  }
}

Widget _buildComment(
  BuildContext context,
  Comment comment,
  bool jumpList,
  String? mode,
  bool gotoComic,
) {
  return InkWell(
    onTap: () {
      if (!jumpList) {
        return;
      }
      if (comment.AID != null) {}
      Navigator.of(context).push(
        MaterialPageRoute(
          builder: (context) => _CommentChildrenScreen(
            aid: comment.AID!,
            mode: mode,
            comment: comment,
          ),
        ),
      );
    },
    child: _ComicCommentItem(
      aid: comment.AID,
      mode: mode,
      comment: comment,
      gotoComic: jumpList && gotoComic,
    ),
  );
}

Widget _buildPostComment(
    BuildContext context, int? parentId, int aid, Function? f) {
  return InkWell(
    onTap: () async {
      String? text = await displayTextInputDialog(context, title: '请输入评论内容');
      if (text != null && text.isNotEmpty) {
        try {
          final data = await (parentId == null
              ? methods.comment(aid, text)
              : methods.childComment(aid, text, parentId));
          if (data.status == "fail") {
            defaultToast(context, data.msg);
          } else {
            defaultToast(context, "评论成功");
            f?.call();
          }
        } catch (e, st) {
          print("$e\n$st");
          defaultToast(context, "评论失败");
        }
      }
    },
    child: Container(
      decoration: BoxDecoration(
        border: Border(
          top: BorderSide(
            width: .25,
            style: BorderStyle.solid,
            color: Colors.grey.shade500.withOpacity(.5),
          ),
          bottom: BorderSide(
            width: .25,
            style: BorderStyle.solid,
            color: Colors.grey.shade500.withOpacity(.5),
          ),
        ),
      ),
      padding: const EdgeInsets.all(30),
      child: const Center(
        child: Text('我有话要讲'),
      ),
    ),
  );
}

class _ComicCommentItem extends StatefulWidget {
  // 清除缓存使用mode和aid
  final String? mode;
  final int? aid;
  final Comment comment;
  final bool gotoComic;

  const _ComicCommentItem({
    required this.mode,
    required this.aid,
    required this.comment,
    required this.gotoComic,
  });

  @override
  State<StatefulWidget> createState() => _ComicCommentItemState();
}

class _ComicCommentItemState extends State<_ComicCommentItem> {
  var likeLoading = false;

  @override
  Widget build(BuildContext context) {
    var comment = widget.comment;
    var theme = Theme.of(context);
    var nameStyle = const TextStyle(fontWeight: FontWeight.bold);
    var levelStyle = TextStyle(
        fontSize: 12, color: theme.colorScheme.secondary.withOpacity(.8));
    var connectStyle =
        TextStyle(color: theme.textTheme.bodyText1?.color?.withOpacity(.8));
    var gotoComicStyle = TextStyle(
      color: theme.colorScheme.secondary.withOpacity(.5),
    );
    var datetimeStyle = TextStyle(
        color: theme.textTheme.bodyText1?.color?.withOpacity(.6), fontSize: 12);
    return Container(
      padding: const EdgeInsets.all(5),
      decoration: BoxDecoration(
        border: Border(
          top: BorderSide(
            width: .25,
            style: BorderStyle.solid,
            color: Colors.grey.shade500.withOpacity(.5),
          ),
          bottom: BorderSide(
            width: .25,
            style: BorderStyle.solid,
            color: Colors.grey.shade500.withOpacity(.5),
          ),
        ),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Avatar(comment.photo),
          Container(width: 5),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                LayoutBuilder(
                  builder: (BuildContext context, BoxConstraints constraints) {
                    return SizedBox(
                      width: constraints.maxWidth,
                      child: Wrap(
                        crossAxisAlignment: WrapCrossAlignment.center,
                        alignment: WrapAlignment.spaceBetween,
                        children: [
                          Text(comment.nickname, style: nameStyle),
                          Text(
                            comment.addtime,
                            style: datetimeStyle,
                          ),
                        ],
                      ),
                    );
                  },
                ),
                Container(height: 3),
                LayoutBuilder(
                  builder: (BuildContext context, BoxConstraints constraints) {
                    return SizedBox(
                      width: constraints.maxWidth,
                      child: Wrap(
                        crossAxisAlignment: WrapCrossAlignment.center,
                        alignment: WrapAlignment.spaceBetween,
                        children: [
                          Text("Lv. ? (?)", style: levelStyle),
                          Text.rich(TextSpan(
                            style: levelStyle,
                            children: [
                              comment.replys.isNotEmpty
                                  ? TextSpan(children: [
                                      WidgetSpan(
                                        alignment: PlaceholderAlignment.middle,
                                        child: Icon(Icons.message,
                                            size: 13,
                                            color: theme.colorScheme.secondary
                                                .withOpacity(.7)),
                                      ),
                                      WidgetSpan(child: Container(width: 5)),
                                      TextSpan(
                                        text: '${comment.replys.length}',
                                      ),
                                    ])
                                  : const TextSpan(),
                              WidgetSpan(child: Container(width: 12)),
                              WidgetSpan(
                                  child: GestureDetector(
                                onTap: () async {
                                  setState(() {
                                    likeLoading = true;
                                  });
                                  // try {
                                  //   switch (widget.mainType) {
                                  //     case CommentMainType.COMIC:
                                  //       await method.switchLikeComment(
                                  //         comment.id,
                                  //         widget.mainId,
                                  //       );
                                  //       break;
                                  //     case CommentMainType.GAME:
                                  //       await method.switchLikeGameComment(
                                  //         comment.id,
                                  //         widget.mainId,
                                  //       );
                                  //       break;
                                  //   }
                                  //   setState(() {
                                  //     if (comment.isLiked) {
                                  //       comment.isLiked = false;
                                  //       comment.likesCount--;
                                  //     } else {
                                  //       comment.isLiked = true;
                                  //       comment.likesCount++;
                                  //     }
                                  //   });
                                  // } catch (e, s) {
                                  //   defaultToast(context, "点赞失败");
                                  // } finally {
                                  //   setState(() {
                                  //     likeLoading = false;
                                  //   });
                                  // }
                                },
                                child: Text.rich(
                                  TextSpan(style: levelStyle, children: [
                                    WidgetSpan(
                                      alignment: PlaceholderAlignment.middle,
                                      child: Icon(
                                        Icons.favorite,
                                        // likeLoading
                                        //     ? Icons.refresh
                                        //     : comment.isLiked
                                        //         ? Icons.favorite
                                        //         : Icons.favorite_border,
                                        size: 13,
                                        color: theme.colorScheme.secondary
                                            .withOpacity(.7),
                                      ),
                                    ),
                                    WidgetSpan(child: Container(width: 5)),
                                    TextSpan(
                                      text: '${comment.likes}',
                                    ),
                                  ]),
                                ),
                              )),
                            ],
                          )),
                        ],
                      ),
                    );
                  },
                ),
                Container(height: 5),
                GestureDetector(
                  onLongPress: () {
                    confirmCopy(context, comment.content);
                  },
                  child: Text(comment.content, style: connectStyle),
                ),
                ...widget.gotoComic
                    ? [
                        Container(height: 8),
                        GestureDetector(
                          onTap: () {
                            if (comment.AID != null) {
                              Navigator.push(context, MaterialPageRoute(
                                  builder: (BuildContext context) {
                                return ComicInfoScreen(comment.AID!, null);
                              }));
                            }
                          },
                          child: LayoutBuilder(
                            builder: (
                              BuildContext context,
                              BoxConstraints constraints,
                            ) {
                              return SizedBox(
                                width: constraints.maxWidth,
                                child: Text(
                                  comment.name,
                                  style: gotoComicStyle,
                                  textAlign: TextAlign.right,
                                ),
                              );
                            },
                          ),
                        ),
                      ]
                    : [],
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _CommentChildrenScreen extends StatefulWidget {
  final String? mode;
  final int aid;
  final Comment comment;

  const _CommentChildrenScreen({
    required this.mode,
    required this.aid,
    required this.comment,
    Key? key,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _CommentChildrenScreenState();
}

class _CommentChildrenScreenState extends State<_CommentChildrenScreen> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: Column(
        children: [
          _ComicCommentItem(
            mode: widget.mode,
            aid: widget.aid,
            comment: widget.comment,
            gotoComic: false,
          ),
          const Divider(),
          Expanded(
            child: ListView(
              children: [
                ...widget.comment.replys.map((e) => _buildComment(
                      context,
                      e,
                      false,
                      widget.mode,
                      false,
                    )),
                _buildPostComment(
                  context,
                  widget.comment.CID,
                  widget.aid,
                  () {
                    // setState(() {
                    //   _future = _loadPage();
                    // });
                  },
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
