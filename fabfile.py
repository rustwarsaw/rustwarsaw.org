# -*- coding: utf-8 -*-

from __future__ import unicode_literals

from fabric.api import *


PROJECT_PATH = "rustwarsaw.org"

env.roledefs = {
    'web': ["rustwarsaw@rustwarsaw.org"],
}


@task
def supervisorctl(command):
    run('supervisorctl {} rustwarsaw_org'.format(command))


@task
@roles("web")
def build_release():
    local("cargo build --release")


@task
@roles("web")
def update_binaries():
    with cd(PROJECT_PATH):
        put("target/release/rustwarsaw", "rustwarsaw")


@task
@roles("web")
def deploy():
    build_release()
    supervisorctl('stop')
    update_binaries()
    supervisorctl('start')
