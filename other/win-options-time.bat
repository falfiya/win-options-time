REM blatantly stolen from https://stackoverflow.com/a/11995662
net session > nul 2>&1
if %errorLevel% == 0 (
   reg add "HKCU\Control Panel\International" /v "s1159" /t REG_SZ /d c /f
   reg add "HKCU\Control Panel\International" /v "s2359" /t REG_SZ /d p /f
   reg add "HKCU\Control Panel\International" /v "sShortTime" /t REG_SZ /d "$SPY hmmtt" /f
) else (
   echo You need to run this script as an Administrator!
)
