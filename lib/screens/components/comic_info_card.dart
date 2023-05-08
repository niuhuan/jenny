import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/entities.dart';
import 'package:jenny/configs/display_jmcode.dart';
import 'package:jenny/screens/comic_search_screen.dart';

import 'images.dart';

class ComicInfoCard extends StatelessWidget {
  final bool link;
  final ComicBasic comic;

  const ComicInfoCard(
    this.comic, {
    this.link = false,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    const titleStyle = TextStyle(fontWeight: FontWeight.bold);
    final authorStyle = TextStyle(
      fontSize: 13,
      color: Colors.pink.shade300,
    );
    return Container(
      padding: const EdgeInsets.only(top: 5, bottom: 5, left: 10, right: 10),
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: Theme.of(context).dividerColor,
          ),
        ),
      ),
      child: Row(
        children: [
          Card(
            shape: coverShape,
            clipBehavior: Clip.antiAlias,
            child: JM3x4Cover(
              comicId: comic.id,
              width: 100 * 3 / 4,
              height: 100,
            ),
          ),
          Container(width: 10),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                ...link
                    ? [
                        Text.rich(TextSpan(children: [
                          TextSpan(
                            text: comic.name,
                            style: titleStyle,
                            recognizer: LongPressGestureRecognizer()
                              ..onLongPress = () {
                                confirmCopy(context, comic.name);
                              },
                          ),
                          ...currentDisplayJmcode()
                              ? [
                                  TextSpan(
                                    text: "  (JM${comic.id})",
                                    style: TextStyle(
                                      fontSize: 13,
                                      color: Colors.orange.shade700,
                                    ),
                                    recognizer: LongPressGestureRecognizer()
                                      ..onLongPress = () {
                                        confirmCopy(context, "JM${comic.id}");
                                      },
                                  ),
                                ]
                              : [],
                        ])),
                      ]
                    : [Text(comic.name, style: titleStyle)],
                Container(height: 4),
                link
                    ? GestureDetector(
                        onTap: () {
                          Navigator.of(context).push(MaterialPageRoute(
                            builder: (BuildContext context) {
                              return ComicSearchScreen(
                                initKeywords: comic.author,
                              );
                            },
                          ));
                        },
                        onLongPress: () {
                          confirmCopy(context, comic.author);
                        },
                        child: Text(comic.author, style: authorStyle),
                      )
                    : Text(comic.author, style: authorStyle),
                Container(height: 4),
                _buildCategoryRow(),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildCategoryRow() {
    if (comic is ComicSimple) {
      var _comic = comic as ComicSimple;
      return Row(
        children: [
          ..._c(_comic.category),
          ..._c(_comic.categorySub),
        ],
      );
    }
    return Container();
  }

  List<Widget> _c(ComicSimpleCategory category) {
    if (category.title == null) {
      return [];
    }
    return [
      Text(category.title!),
      Container(width: 15),
    ];
  }
}
