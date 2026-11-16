# Attachment picker — final Lab revision, 2026-11-17

Branch `fix/attachment-picker-regions-20261117`, base main `84e75c57`.

The panel and nested file/directory content overlays own an opaque
`canvas-solid` surface. Its body has three spaced, aligned sections:

1. 选择文件: local file/directory actions on the first row; other machines'
   avatars and names on the second. Only complete chips that fit remain inline;
   the rest are selectable in the More menu. The local machine is not duplicated.
2. 已选文件: hidden entirely when empty. Selected rows share AttachmentList and
   FileChoiceRow with the existing workspace geometry and size formatter.
3. 发送目录: follows selection directly, with the shared avatar/colon/path row.

Short keyboard panels scroll their body instead of squeezing controls. Long
selected lists retain independent scrolling. Missing native-path sizes are read
from the source through stat_text without staging, uploading, changing selection
or blocking send. Known sizes are reused; unknown directory totals stay unknown.
Desktop terminal's paperclip HeaderSeat is removed. Direct browser/native/internal
drops open the draft; Mobile retains its key row attachment entry.

Both final realApp runs43914447734/e12594ec and43915117547/c0264748 passed14
Chromium/WebKit attachment/GUI/Khor scenarios at1280/412/390/320, builds, unit,
token/mobile static and hot-update checks. The latter additionally forces80px
machine room, checks every overflow machine is reachable and restores inline
choices; it checks empty-section removal, source row separation, all three
section boundaries/order/alignment and nonoverlapping button content. Existing
file size/source, preview, cancel/send, scrolling,232px touch row and unchanged
PTY dimension checks remain. Desktop and narrow screenshots were inspected.

Live App: http://127.0.0.1:23961/app/?scene=terminal&machines=3
Pinned App: http://127.0.0.1:23961/snapshots/c026474864e655bd/app/?scene=terminal&machines=3
Desktop: drag a file into the terminal. Mobile: use the attachment key; scroll
panel content to reach the destination when the keyboard area is short.

Artifact14251060354 SHA2566933ef468c03d5c2c82907835448a1ad78a40738ef14f0e4cabde301106321ce
has verified ZIP CRC, exact sourcec0264748 and tree2beaf89d43335bb9d6f0471f836d8970a5763c4a,
14 error-free scenarios and passed hot-update checks. Only the verified owned
previous Lab server was replaced; historical snapshot URLs remain available.
The final documentation commit leaves tested App/native/crate sources unchanged.

This follow-up is not merged or shipped. Installed platform file pickers and
native drag feel remain separate actual-device checks.
