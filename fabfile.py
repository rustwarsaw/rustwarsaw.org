# -*- coding: utf-8 -*-

from __future__ import unicode_literals

from fabric.api import *
from fabric.contrib.project import rsync_project


PROJECT_PATH = "rustwarsaw.org"

env.roledefs = {
    'web': ["rustwarsaw@rustwarsaw.org"],
}
env.use_ssh_config = True


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
    rsync_project("{}/www/".format(PROJECT_PATH), "www/", delete=True)


@task
@roles("web")
def deploy():
    build_release()
    supervisorctl('stop')
    update_binaries()
    supervisorctl('start')
