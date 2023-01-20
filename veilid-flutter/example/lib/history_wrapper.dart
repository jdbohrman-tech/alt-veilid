import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

// TextField History Wrapper
class HistoryWrapper {
  final List<String> _history = [];
  int _historyPosition = 0;
  final _historyTextEditingController = TextEditingController();
  String _historyCurrentEdit = "";

  TextEditingController get controller {
    return _historyTextEditingController;
  }

  void submit(String v) {
    // add to history
    if (_history.isEmpty || _history.last != v) {
      _history.add(v);
      if (_history.length > 100) {
        _history.removeAt(0);
      }
    }
    _historyPosition = _history.length;
    _historyTextEditingController.text = "";
  }

  Widget wrap(
      void Function(void Function())? stateSetter, TextField textField) {
    void Function(void Function()) setState = stateSetter ?? (x) => x();
    return KeyboardListener(
      onKeyEvent: (KeyEvent event) {
        setState(() {
          if (event.runtimeType == KeyDownEvent &&
              event.logicalKey == LogicalKeyboardKey.arrowUp) {
            if (_historyPosition > 0) {
              if (_historyPosition == _history.length) {
                _historyCurrentEdit = _historyTextEditingController.text;
              }
              _historyPosition -= 1;
              _historyTextEditingController.text = _history[_historyPosition];
            }
          } else if (event.runtimeType == KeyDownEvent &&
              event.logicalKey == LogicalKeyboardKey.arrowDown) {
            if (_historyPosition < _history.length) {
              _historyPosition += 1;
              if (_historyPosition == _history.length) {
                _historyTextEditingController.text = _historyCurrentEdit;
              } else {
                _historyTextEditingController.text = _history[_historyPosition];
              }
            }
          } else if (event.runtimeType == KeyDownEvent) {
            _historyPosition = _history.length;
            _historyCurrentEdit = _historyTextEditingController.text;
          }
        });
      },
      focusNode: FocusNode(onKey: (FocusNode node, RawKeyEvent event) {
        if (event.logicalKey == LogicalKeyboardKey.arrowDown ||
            event.logicalKey == LogicalKeyboardKey.arrowUp) {
          return KeyEventResult.handled;
        }
        return KeyEventResult.ignored;
      }),
      child: textField,
    );
  }
}
