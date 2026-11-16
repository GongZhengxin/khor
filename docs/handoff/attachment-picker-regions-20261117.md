# Attachment sources and selected files — 2026-11-17

Branch `fix/attachment-picker-regions-20261117`, base main `84e75c57`.

The attachment keyboard panel inherited `background`, which is transparent in
a translucent native window. Its source/selected sections had only aria labels,
so selected files visually looked like children of the preceding machine entry.
Native path drops supplied no file size.

The panel and nested file/directory content overlays now own an opaque
`canvas-solid` surface. Visible “选择文件” and “已选文件” sections distinguish
sources from selected rows. Compact source choices leave a full file touch row
in a 232-point keyboard panel. Selected files reuse AttachmentList/FileChoiceRow;
size is separate from truncatable source detail, using the existing size format.
Missing native-path sizes are read from their source machine through stat_text,
without staging/uploading, changing the selection or blocking send. Known sizes
are reused; unknown directory totals are not invented.

Desktop terminal's top paperclip HeaderSeat was removed at the user's request.
Direct browser/native/internal drops still open the draft; Mobile keeps its key
row attachment entry. File open/preview actions remain on selected rows.

Validation: red118904d3/run43906615619 reproduces fully transparent desktop
background in both browser engines and missing visible section headings. Final
3ca35b77/run43909943576 passes builds, unit/token/mobile checks and14 actualApp
attachment/GUI/Khor scenarios, both Chromium and WebKit at1280/412/390/320 widths,
including source-specific path sizes, no premature upload, preview, deselection,
send, independent scrolling, short keyboard height and unchanged PTY dimensions.
Source hot-update checks passed. The first candidate's font token and subsequent
short-panel spacing failures were corrected without weakening the old checks.

Live complete App: http://127.0.0.1:23961/app/?scene=terminal&machines=3 .
Pinned App: http://127.0.0.1:23961/snapshots/3ca35b77bdd0c870/app/?scene=terminal&machines=3 .
Desktop: drag a file into the terminal. Mobile: use the key row attachment entry.
Artifact14248021372 SHA25618c12c4d69ebf4baec9cc5767c92aa0fbcb408507ce66d75a445c7e261f8f82d
was verified against source3ca35b77 and tree81b9b1fb3353ed470e8666dd7a7a33c97098ba14;
ZIP CRC,14 error-free scenarios and hot-update proof all passed. Desktop/mobile
screenshots were inspected. LivePID95749/exec36313; only old owned27775 replaced
after argv/source confirmation. Four entry assets byte-verified and35 historical
snapshot URLs retained200. This follow-up has not been merged or shipped; the preceding Air
access/glass follow-up is already on main. Native drag feel and actual platform
file pickers remain separate installed-App checks; Web Lab does not claim them.
