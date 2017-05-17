class Feed():
    def __init__(self, channel):
        self.channel = channel

    def gen(self):
        return '\n'.join([
            '<?xml version="1.0" encoding="UTF-8" ?>',
            '<rss version="2.0">',
            self.channel.gen(),
            '</rss>'])

class Channel():
    def __init__(self, title, link, description, items):
        self.title = title
        self.link = link
        self.description = description
        self.items = items

    def gen(self):
        return '\n'.join([
            '<channel>',
            '<title>', self.title, '</title>',
            '<link>', self.link, '</link>',
            '<description>', self.description, '</description>',
            *[item.gen() for item in self.items],
            '</channel>'])

class Item():
    def __init__(self, title, link, description):
        self.title = str(title)
        self.link = str(link)
        self.description = str(description)

    def gen(self):
        return '\n'.join([
            '<item>',
            '<title>', self.title, '</title>',
            '<link>', self.link, '</link>',
            '<description>', self.description, '</description>',
            '</item>'])
