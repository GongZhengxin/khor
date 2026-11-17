@rem Khor shell integration for cmd.exe, loaded by /k after native AutoRun.
@rem Preserve the user's configured prompt; cmd has no universal preexec hook.
@if defined KHOR_CMD_INTEGRATED exit /b
@if not defined PROMPT set "PROMPT=$P$G"
@set "PROMPT=$e]133;D$e\$e]133;A$e\$e]7;file://%COMPUTERNAME%/$P$e\%PROMPT%$e]133;B$e\"
@set "KHOR_CMD_INTEGRATED=1"
