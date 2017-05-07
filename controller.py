import tornado.web
import traceback
import entity
import datetime

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

    args = {}

    def write_error(self, status_code, **kwargs):
        ''' Handler基类捕获所有异常，通过自定义界面渲染 '''
        article = entity.Article()
        tmp = kwargs.get('exc_info')
        error = tmp[0]
        title = tmp[1]
        article.title = title
        article.brief = error
        article.content = traceback.format_exc()
        article.date = datetime.datetime.now()
        article.label = 'error'
        self.args.update({
            'title': title,
            'article': article,
        })
        self.render('article.html', **self.args)

    def post(self):
        self.get(self)

    def page_helper(self):
        self.args.update({'pages':int(self.query.count()/10)})
        page = int(self.get_argument('page', 1))
        self.args.update({'page':page})
        self.args.update({'cards':self.query.paginate(page, 10)})

@route(r'/')
class Root(Handler):
    def get(self):
        self.redirect('/home')

@route(r'/home')
class Home(Handler):
    def get(self):
        self.query = (entity.Article.select()
                .order_by(entity.Article.date.desc()))
        self.page_helper()
        self.args.update({'title': 'Welcome to Koumakan'})
        self.render('home.html', **self.args)

@route(r'/type/(\w+)')
class Type(Handler):
    def get(self, param):
        self.query = (entity.Article.select()
                .where(entity.Article.type==param)
                .order_by(entity.Article.date.desc()))
        self.page_helper()
        self.args.update({'title': param + ' 分类下的文章'})
        self.render('home.html', **self.args)

@route(r'/list')
class ListByTime(Handler):
    def get(self):
        param = self.get_argument('param')
        self.args.update({
            'title': param + ' 的搜索结果',
            'cards': []
        })
        self.render('list.html', **args)

@route(r'/search')
class Search(Handler):
    def get(self):
        param = self.get_argument('param')
        self.query = (entity.Article.select().where(
            entity.Article.type.contains(param)|
            entity.Article.title.contains(param)|
            entity.Article.brief.contains(param)|
            entity.Article.label.contains(param))
            .order_by(entity.Article.date.desc()))
        self.page_helper()
        self.args.update({'title': param + ' 的搜索结果'})
        self.render('home.html', **self.args)

@route(r'/article/(\d+)')
class Article(Handler):
    def get(self, article_id):
        article = entity.Article.get(entity.Article.id==article_id)
        self.args.update({
            'title': article.title,
            'article': article
        })
        self.render('article.html', **self.args)

@route(r'/rss')
class RSS(Handler):
    def get(self):
        self.write("rss")

@route(r'.*')
class PageNotFoundHandler(Handler):
    ''' 通过最后映射通用路径捕获404错误 '''
    def get(self):
        raise tornado.web.HTTPError(404)
