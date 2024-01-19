(() => {
  // Origin: https://github.com/SARDONYX-sard/dar-to-oar/blob/main/locales/en-US.json
  const i18n = {
    'all-clear-btn': '全部清除',
    'conversion-complete': '转换完成',
    'convert-btn': '转换',
    'convert-form-author-name': 'Mod作者名',
    'convert-form-author-name-helper': '[可选]',
    'convert-form-author-placeholder': '作者名',
    'convert-form-dar-helper': '[必填] 包含"DynamicAnimationReplacer"的目录',
    'convert-form-dar-helper2': '"C:\\[...]/Mod Name/" -> 转换第一人称和第三人称',
    'convert-form-dar-helper3': '"[...]/animations/DynamicAnimationReplacer" -> 仅转换第三人称',
    'convert-form-dar-label': 'DAR(src)目录',
    'convert-form-mapping-1st-label': '映射表路径(第一人称用)',
    'convert-form-mapping-help-link-name': '什么是映射文件？',
    'convert-form-mapping-helper': '[可选] 指定包含优先级和部分名称对应关系的文件',
    'convert-form-mapping-helper2': '帮助: ',
    'convert-form-mapping-label': '映射表路径',
    'convert-form-mod-name': 'Mod名称',
    'convert-form-mod-name-helper': '[可选] 推荐使用ASCII(英文)',
    'convert-form-oar-helper': '[可选] 指定OAR的输出目录(例如: "NewMod" -> "NewMod/meshes/[...])"',
    'convert-form-oar-helper2': '如果未指定，则OAR将在与DAR相同的位置创建',
    'convert-form-oar-label': 'OAR(dist)目录',
    'converting-btn': '转换中...',
    'css-preset-list-item0': '自定义',
    'css-preset-list-item1': '预设1',
    'css-preset-list-item2': '预设2',
    'css-preset-list-item3': '预设3',
    'css-preset-list-item4': '预设4',
    'css-preset-list-label': '预设',
    'css-preset-list-tooltip': '选择CSS预设',
    'css-preset-list-tooltip2': '注意: 编辑"预设"会覆盖"自定义"',
    'custom-css-label': '当前应用的CSS',
    'custom-js-label': '自定义JavaScript(请勿执行不可信脚本)',
    'editor-mode-list-label': '编辑模式',
    'hide-dar-btn': '隐藏DAR',
    'hide-dar-btn-tooltip': '在转换后，将".mohidden"添加到DAR的所有文件中以隐藏它们(MO2用户专用)',
    'hide-dar-btn-tooltip2': '信息: 如果未指定OAR的输出目录，则这很方便。',
    'import-lang-btn': '导入语言',
    'import-lang-tooltip': '从Json文件导入任何语言。(启用后会自动重新加载)',
    'import-lang-tooltip2': '注意: 如果Json无效，将回退到英语。(请参考Wiki了解Json的写法)',
    'log-level-list-label': '日志级别',
    'log-level-list-tooltip': '较轻的日志级别包含更严重的日志级别。(Error ⊂ Info)',
    'log-level-list-tooltip2': 'Debug: 记录转换条件的中间数据',
    'log-level-list-tooltip3': 'Info: 记录转换时间',
    'log-level-list-tooltip4': 'Error: 除了严重错误外不记录',
    'mapping-wiki-url-leaf': 'wiki#what-is-the-mapping-file',
    'open-log-btn': '查看日志',
    'open-log-dir-btn': '打开日志(目录)',
    'open-log-dir-tooltip': '打开日志的存储位置。',
    'open-log-tooltip': '打开当前的日志文件。(每次启动应用程序时都会轮换到新的日志文件)',
    'progress-btn': '进度条',
    'progress-btn-tooltip': '显示详细的进度信息',
    'progress-btn-tooltip2': '',
    'remove-oar-btn': '删除OAR',
    'remove-oar-failed': '未找到"OpenAnimationReplacer"目录',
    'remove-oar-specify-error': '未输入DAR或OAR',
    'remove-oar-success': '已删除OAR目录',
    'remove-oar-tooltip': '从OAR(dist)(如果不存在则从DAR(src))中搜索并删除OAR目录',
    'run-parallel-btn-tooltip': '使用多线程',
    'run-parallel-btn-tooltip2':
      '注意: 可以期望2倍以上的处理速度，但由于并行处理，线程的结束时间可能是无序的，因此日志写入也是无序的，降低了日志的可读性。',
    'run-parallel-label': '并行执行',
    'select-btn': '选择',
    'unhide-dar-btn': '解除DAR隐藏',
    'unhide-dar-btn-tooltip': '取消"隐藏DAR"以取消隐藏(MO2用户专用)',
    'unhide-dar-failed': '未找到带有扩展名".mohidden"的文件',
    'unhide-dar-specify-error': '请指定DAR(src)',
    'unhide-dar-success': '已解除DAR的隐藏',
  };

  const setCustomTranslation = () => {
    localStorage.setItem('custom-translation-dict', JSON.stringify(i18n));
    localStorage.setItem('locale', 'custom');
  };
  const clearCustomTranslation = () => localStorage.removeItem('custom-translation-dict');

  // You can turn them on and off by deleting the `//` in these functions.
  // Perhaps you may have to reload the file twice to apply it.
  // Key to reload: Ctrl + Shift + r
  // Comment-out toggle key: Ctrl + /
  //
  // clearCustomTranslation();
  // setCustomTranslation();
})();
