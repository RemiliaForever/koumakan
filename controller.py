import tornado.web
import traceback
from entity import *

URLMap = list()
def route(url):
    def decorator(handler):
        URLMap.append((url,handler))
        print('mapping %s -> %s' %(url.ljust(20), handler))
        return handler
    return decorator

class Handler(tornado.web.RequestHandler):
    ''' Handler类基类，实现自定义异常捕获和通用函数 '''
#    def prepare(self):
#        ''' 重定向http到https '''
#        if self.request.protocol == 'http':
#            self.redirect('https://' + self.request.host, permanent=False)

    def get_navigate(self):
        ''' 获取导航栏信息 '''
        pass

    def write_error(self, status_code, **kwargs):
        ''' Handler基类捕获所有异常，通过自定义界面渲染 '''
        tmp = kwargs.get('exc_info')
        error = tmp[0]
        title = tmp[1]
        message = traceback.format_exc()
        message = message + "<a href='www.baidu.com'>asfdfs</a>"
        args = {
            'title': title,
            'error': error,
            'nav': self.get_navigate()
        }
        self.render('error.html', **args)

    def post(self):
        self.get(self)

@route(r'/')
class root(Handler):
    def get(self):
        self.redirect('/home')

@route(r'/home')
class home(Handler):
    def get(self):
        args = {
            'title': 'Welcome to Koumakan',
            'cards': list(Article.select())
        }
        self.render('home.html', **args)

@route(r'/type/(\w+)')
class type(Handler):
    def get(self, param):
        args = {
            'title': param + '分类下的文章',
            'cards': list(Article.select())
        }
        self.render('home.html', **args)

@route(r'/article/(\d+)')
class article(Handler):
    def get(self, article_id):
        self.render('article.html')

@route(r'.*')
class PageNotFoundHandler(Handler):
    ''' 通过最后映射通用路径捕获404错误 '''
    def get(self):
        raise tornado.web.HTTPError(404)
