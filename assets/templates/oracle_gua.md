
## 卦意
*{{ summary }}*

## 卦辭
    {{ guaci }}

{% if guaci_explain %}
## 卦辭解
{% for line in guaci_explain %}
* {{ line }}
{% endfor %}
{% endif %}
## 易傳
***{{ tuan }}***

## 釋義
{% for line in tuan_explain %}
* {{ line }}
{% endfor %}
## 易傳
***{{ daxiang }}***

## 釋義
{% for line in daxiang_explain %}
* {{ line }}
{% endfor %}
## 占斷
{% for line in guazhan %}
󰚀 {{ line }}
{% endfor %}