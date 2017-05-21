from peewee import *
import json

database = MySQLDatabase('koumakan', **json.loads(
    ''.join(open('settings.json').readlines())))


class BaseModel(Model):
    class Meta:
        database = database


class Article(BaseModel):
    brief = TextField(null=True)
    content = TextField(null=True)
    date = DateTimeField(null=True)
    label = CharField(index=True, null=True)
    title = CharField(index=True, null=True)
    type = CharField(null=True)

    class Meta:
        db_table = 'article'


class Comment(BaseModel):
    article = ForeignKeyField(db_column='article_id', null=True, rel_model=Article, to_field='id')
    content = TextField(null=True)
    name = CharField(null=True)
    email = CharField(null=True)
    avatar = CharField(null=True)
    website = CharField(null=True)
    date = DateTimeField(null=True)

    class Meta:
        db_table = 'comment'
