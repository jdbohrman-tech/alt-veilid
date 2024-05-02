import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:xterm/xterm.dart';
import 'log.dart';
import 'veilid_theme.dart';

const kDefaultTerminalStyle = TerminalStyle(
    fontSize: kDefaultMonoTerminalFontSize,
    fontFamily: kDefaultMonoTerminalFontFamily);

class LogTerminal extends StatefulWidget {
  const LogTerminal({super.key});

  @override
  // ignore: library_private_types_in_public_api
  State<LogTerminal> createState() => _LogTerminalState();
}

class _LogTerminalState extends State<LogTerminal> {
  final _terminal = Terminal(
    maxLines: 10000,
  );

  final _terminalController = TerminalController();

  @override
  void initState() {
    super.initState();
    _terminal.setLineFeedMode(true);
    globalTerminalPrinter.setCallback((log) {
      _terminal.write('${log.pretty()}\n');
    });
  }

  @override
  Widget build(BuildContext context) => TerminalView(
        _terminal,
        textStyle: kDefaultTerminalStyle,
        controller: _terminalController,
        autofocus: true,
        backgroundOpacity: 0.9,
        onSecondaryTapDown: (details, offset) async {
          final selection = _terminalController.selection;
          if (selection != null) {
            final text = _terminal.buffer.getText(selection);
            _terminalController.clearSelection();
            await Clipboard.setData(ClipboardData(text: text));
          } else {
            final data = await Clipboard.getData('text/plain');
            final text = data?.text;
            if (text != null) {
              _terminal.paste(text);
            }
          }
        },
      );
}
