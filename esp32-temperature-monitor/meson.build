project('esp32-temperature-monitor', license : 'GPLv3')

fs = import('fs')

wokwi_simulator = find_program('wokwi-cli')

run_command('cargo', 'build')

wokwi_token_file = files('wokwi-token.txt')
wokwi_token = fs.read(wokwi_token_file).strip()
wokwi_token_env_var = {'WOKWI_CLI_TOKEN': wokwi_token}

wokwi_simulator_workdir = meson.current_source_dir()
wokwi_timeout_defaults = ['--timeout', '1000']

wokwi_simulator_args = wokwi_timeout_defaults + ['--expect-text', 'INFO - Hello you!']
test('Test simulator works as expected', wokwi_simulator, args: wokwi_simulator_args, env: wokwi_token_env_var, workdir: wokwi_simulator_workdir)

temperature_test_args = wokwi_timeout_defaults + ['--expect-text', 'INFO - Temperature: 60F']
test('Test temperature is being read correctly.', wokwi_simulator, args: temperature_test_args, env: wokwi_token_env_var, workdir: wokwi_simulator_workdir)
