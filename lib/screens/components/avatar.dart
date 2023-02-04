import 'package:flutter/material.dart';
import 'package:jenny/screens/components/images.dart';

const double _avatarMargin = 5;
const double _avatarBorderSize = 1.5;

// 头像
class Avatar extends StatelessWidget {
  final String photoName;
  final double size;

  const Avatar(this.photoName, {this.size = 50, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context);
    return Container(
      margin: const EdgeInsets.all(_avatarMargin),
      decoration: BoxDecoration(
          shape: BoxShape.circle,
          border: Border.all(
            color: theme.colorScheme.secondary,
            style: BorderStyle.solid,
            width: _avatarBorderSize,
          )),
      child: ClipRRect(
        borderRadius: BorderRadius.all(Radius.circular(size)),
        child: photoName == "?v=0?v="
            ? Icon(
                Icons.query_builder_sharp,
                color: Colors.white,
                size: size,
              )
            : JMPhotoImage(
                photoName: photoName,
                width: size,
                height: size,
              ),
      ),
    );
  }
}
