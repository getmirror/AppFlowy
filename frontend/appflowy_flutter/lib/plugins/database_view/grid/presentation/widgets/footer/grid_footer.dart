import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/generated/locale_keys.g.dart';
import 'package:appflowy/plugins/database_view/grid/application/grid_bloc.dart';
import 'package:appflowy/plugins/database_view/grid/presentation/layout/sizes.dart';
import 'package:appflowy_editor/appflowy_editor.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:flowy_infra/theme_extension.dart';

import 'package:flowy_infra_ui/style_widget/button.dart';
import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class GridAddRowButton extends StatelessWidget {
  const GridAddRowButton({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final color =
        PlatformExtension.isMobile ? null : Theme.of(context).hintColor;
    return FlowyButton(
      text: FlowyText.medium(
        LocaleKeys.grid_row_newRow.tr(),
        color: color,
      ),
      hoverColor: AFThemeExtension.of(context).lightGreyHover,
      onTap: () => context.read<GridBloc>().add(const GridEvent.createRow()),
      leftIcon: FlowySvg(
        FlowySvgs.add_s,
        color: color,
      ),
    );
  }
}

class GridRowBottomBar extends StatelessWidget {
  const GridRowBottomBar({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: GridSize.footerContentInsets,
      height: GridSize.footerHeight,
      margin: const EdgeInsets.only(bottom: 200, top: 8),
      child: const GridAddRowButton(),
    );
  }
}
