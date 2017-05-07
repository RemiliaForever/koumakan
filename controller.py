import tornado.web
import traceback
import entity
import datetime

import pdb

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
        article.title = f'HTTP {status_code}'
        article.brief = None
        article.content = '<pre><code class="python">'+'\n'.join(traceback.format_exception(*kwargs['exc_info']))+'</code></pre>'
        article.date = datetime.datetime.now()
        article.label = 'error'
        article.type = 'error'
        article.icon = 'error'
        self.args.update({
            'title': article.title,
            'article': article,
            'comments': 'tag for error page',
            'pre_article': None,
            'next_article': None
        })
        self.render('article.html', **self.args)

    def post(self):
        self.get(self)

    TIMap = {
        'IT':'phonelink',
        'ACG':'games',
        'ABOUT':'link'
    }

    def page_helper(self):
        self.args.update({'pages':int(self.query.count()/10)})
        page = int(self.get_argument('page', 1))
        pagesize = int(self.get_cookie('pagesize', 10))
        self.args.update({'page':page})
        self.args.update({'cards':self.query.paginate(page, pagesize)})
        for card in self.args.get('cards'):
            card.icon = self.TIMap.get(card.type)

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
        article_id = int(article_id)
        try:
            article = entity.Article.get(entity.Article.id==article_id)
        except entity.Article.DoesNotExist:
            raise tornado.web.HTTPError(404)
        try:
            pre_article = entity.Article.get(entity.Article.id==article_id-1)
        except entity.Article.DoesNotExist:
            pre_article = None
        try:
            next_article = entity.Article.get(entity.Article.id==article_id+1)
        except entity.Article.DoesNotExist:
            next_article = None
        try:
            comments = list(entity.Comment.select()
                    .where(entity.Comment.article==article)
                    .order_by(entity.Comment.date.desc()))
        except entity.Comment.DoesNotExist:
            comments = None
        article.icon = self.TIMap.get(article.type)
        self.args.update({
            'title': article.title,
            'article': article,
            'comments': comments,
            'pre_article': pre_article,
            'next_article': next_article
        })
        self.render('article.html', **self.args)

@route(r'/comment_post')
class CommentPost(Handler):
    def post(self):
        comment = entity.Comment()
        comment.name = self.get_argument('name').strip()
        if comment.name == '':
            self.write('Name could not be null!')
            return
        comment.email = self.get_argument('email').strip()
        if comment.email == '':
            self.write('Email could not be null!')
            return
        comment.website = self.get_argument('website').strip()
        comment.article = self.get_argument('id').strip()
        comment.content = self.get_argument('comment').strip()
        comment.date = datetime.datetime.now()
        if comment.content == '':
            self.write('Comment could not be null!')
            return
        comment.save()
        self.write(f'success')

    def write_error(self, status_code, **kwargs):
        self.write(f'Post Failed! Error Code {status_code}')

@route(r'/rss')
class RSS(Handler):
    def get(self):
        self.write("rss")

@route(r'.*')
class PageNotFoundHandler(Handler):
    ''' 通过最后映射通用路径捕获404错误 '''
    def get(self):
        raise tornado.web.HTTPError(404)
