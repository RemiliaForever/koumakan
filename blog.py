#!/bin/python

import os
import signal
import datetime
import logging
import tornado.httpserver
import tornado.ioloop
import tornado.options
import tornado.web

import controller

settings = {
    'static_path': os.path.join(os.path.dirname(__file__), 'static'),
    'template_path': os.path.join(os.path.dirname(__file__), 'template'),
    'cookie_secret': 'zY0BmjroRDmhRmyFiQIYOQawZRgJv0wVimB31EvtEX4=',
    'debug': True,
    'xheader': True,
    "autoescape":None
}


def on_kill(*_):
    tornado.log.app_log.info('progress stop')
    os._exit(0)



if __name__ == "__main__":
    tornado.options.parse_command_line()
    signal.signal(signal.SIGINT, on_kill)
    app = tornado.web.Application(handlers=controller.URLMap, **settings)
    http_server = tornado.httpserver.HTTPServer(app)
    http_server.listen(443)
    http_server.listen(80)
    controller.database.connect()
    tornado.ioloop.IOLoop.instance().start()

