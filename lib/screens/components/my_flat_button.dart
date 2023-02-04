import 'package:flutter/material.dart';

class MyFlatButton extends StatelessWidget {
  final String title;
  final VoidCallback? onPressed;

  const MyFlatButton({
    required this.title,
    required this.onPressed,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
        builder: (BuildContext context, BoxConstraints constraints) {
      var width = constraints.maxWidth;
      return Container(
        padding: const EdgeInsets.only(left: 10, right: 10),
        margin: const EdgeInsets.only(bottom: 10),
        width: width,
        child: MaterialButton(
          onPressed: onPressed,
          child: Row(
            children: [
              Expanded(
                child: Container(
                  color: Theme.of(context)
                      .textTheme
                      .bodyText1!
                      .color!
                      .withOpacity(.05),
                  padding: const EdgeInsets.all(10),
                  child: Text(
                    title,
                    textAlign: TextAlign.center,
                  ),
                ),
              )
            ],
          ),
        ),
      );
    });
  }
}
