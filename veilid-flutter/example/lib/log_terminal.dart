import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:xterm/xterm.dart';
import 'log.dart';
import 'veilid_theme.dart';

const kDefaultTerminalStyle = TerminalStyle(
    fontSize: kDefaultMonoTerminalFontSize,
    height: kDefaultMonoTerminalFontHeight,
    fontFamily: kDefaultMonoTerminalFontFamily);

class LogTerminal extends StatefulWidget {
  const LogTerminal({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _LogTerminalState createState() => _LogTerminalState();
}

class _LogTerminalState extends State<LogTerminal> {
  final terminal = Terminal(
    maxLines: 10000,
  );

  final terminalController = TerminalController();

  @override
  void initState() {
    super.initState();
    terminal.setLineFeedMode(true);
    globalTerminalPrinter
        .setCallback((log) => {terminal.write("${log.pretty()}\n")});
  }

  @override
  Widget build(BuildContext context) {
    return TerminalView(
      terminal,
      textStyle: kDefaultTerminalStyle,
      controller: terminalController,
      autofocus: true,
      backgroundOpacity: 0.9,
      onSecondaryTapDown: (details, offset) async {
        final selection = terminalController.selection;
        if (selection != null) {
          final text = terminal.buffer.getText(selection);
          terminalController.clearSelection();
          await Clipboard.setData(ClipboardData(text: text));
        } else {
          final data = await Clipboard.getData('text/plain');
          final text = data?.text;
          if (text != null) {
            terminal.paste(text);
          }
        }
      },
    );
  }
}
