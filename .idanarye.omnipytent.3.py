from omnipytent import *
from omnipytent.execution import ShellCommandExecuter
from omnipytent.integration.plumbum import local


@ShellCommandExecuter
def ERUN(command):
    CMD.Erun.bang(command)

cargo = local['cargo']


@task
def compile(ctx):
    local['cargo']['build', '-q'] & ERUN


@task
def run(ctx):
    cargo['run'] & BANG


@task
def test(ctx):
    cargo['test'] & BANG


@task
def clean(ctx):
    cargo['clean'] & BANG

