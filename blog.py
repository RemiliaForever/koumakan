#!/usr/bin/python

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
    'xheader': True,
    'debug':True
    #"autoescape":None
}

def on_kill(*_):
    tornado.log.app_log.info('progress stop')
    os._exit(0)

class MainHandler(tornado.web.RequestHandler):
    def prepare(self):
        if self.request.protocol == 'http':
            self.redirect('https://' + self.request.host + self.request.uri, permanent=False)

if __name__ == "__main__":
    tornado.options.parse_command_line()
    signal.signal(signal.SIGINT, on_kill)
    app = tornado.web.Application(handlers=controller.URLMap, **settings)
    http_server = tornado.httpserver.HTTPServer(app, ssl_options={
        'certfile':os.path.join(os.path.dirname(__file__), 'cert'),
        'keyfile': os.path.join(os.path.dirname(__file__), 'key')
    })
    http_server.listen(443)
    application = tornado.web.Application([(r'.*', MainHandler)])
    application.listen(80)
    controller.entity.database.connect()
    tornado.ioloop.IOLoop.instance().start()
