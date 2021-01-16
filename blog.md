---
layout: page
title: Blog
---

<ul>
    {% for post in site.posts %}
      <li>
        <a href="{{ site.baseurl }}{{ post.url }}">{{ post.title }}</a>
        {% for tag in post.tags %}
          <i>#{{ tag }}</i>
        {% endfor %}
      </li>
    {% else %}
    No posts yet.
    {% endfor %}
</ul>
