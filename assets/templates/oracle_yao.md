## 爻辭
    {{ yaoci }}

## 爻辭解
{% for line in yaoci_explain %}
* {{ line }}
{% endfor %}
## 易傳
***{{ xiaoxiang }}***

## 釋義
{% for line in yaoci_explain %}
* {{ line }}
{% endfor %}
## 占斷
{% for line in yaozhan %}
󰚀 {{ line }}
{% endfor %}
## 案例
{% for case in cases %}
󰚀 {{ case.Q }}
{% for line in case.A %}
> {{ line }}
{% endfor %}
{% endfor %}