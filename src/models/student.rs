use serde::Deserialize;

#[derive(Clone, PartialEq,Debug, Deserialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub topic: String,
    pub supervisors: Vec<String>,
    pub start_date: String,
    pub scholarship: String 
}

pub const STUDENTS_JSON: &str = r#"
[
    {
        "id": 1,
        "name": "Ziyi Sun",
        "topic": "Evolutionary Computation for Image Analysis",
        "supervisors": ["Prof Bing Xue", "Prof Mengjie Zhang"],
        "start_date": "1 Oct 2021",
        "scholarship": "Wellington Doctoral Scholarship"
    },
    {
        "id": 2,
        "name": "Jordan MacLachlan",
        "topic": "Evolutionary Computation for Emergency Medical Services Routing",
        "supervisors": ["Dr Yi Mei", "Dr Fangfang Zhang", "Prof Mengjie Zhang"],
        "start_date": "1 Oct 2021",
        "scholarship": "Wellington Doctoral Scholarship"
    },
    {
        "id": 3,
        "name": "Dylon Zeng",
        "topic": "Evolutionary Computation for Image Analysis",
        "supervisors": ["Prof Mengjie Zhang", "Ivy Liu", "Prof Bing Xue"],
        "start_date": "1 Oct 2021",
        "scholarship": "Wellington Doctoral Scholarship"
    },
    {
        "id": 4,
        "name": "Qinyu Wang",
        "topic": "Evolutionary Computation for image analysis",
        "supervisors": ["Prof Bing Xue", "Prof Mengjie Zhang"],
        "start_date": "1 Sep 2021",
        "scholarship": "China Scholarship Council"
    },
    {
        "id": 5,
        "name": "Kaan Demir",
        "topic": "Sparsity-based Dimensionality Reduction for Multi-label Classification",
        "supervisors": ["Dr Bach Nguyen", "Prof Bing Xue"],
        "start_date": "1 April 2021",
        "scholarship": "Wellington Doctoral Scholarship"
    },
    {
        "id": 6,
        "name": "Christian Raymond",
        "topic": "Meta Learning for Loss Function Learning",
        "supervisors": ["Dr Qi Chen", "Prof Bing Xue"],
        "start_date": "1 March 2021",
        "scholarship": "Wellington Doctoral Scholarship"
    },
    {
        "id": 7,
        "name": "Hayden Andersen",
        "topic": "Evolutionary Computation for Creating Human-friendly Explanations",
        "supervisors": ["Dr Andrew Lensen", "Dr Yi Mei", "Prof Will Browne"],
        "start_date": "1 March 2021",
        "scholarship": "Wellington Doctoral Scholarship"
    },
    {
        "id": 8,
        "name": "Jiabin Lin",
        "topic": "Evolutionary Transfer Learning for Feature Selection in Classification",
        "supervisors": ["Dr Qi Chen", "Prof Bing Xue", "Prof Mengjie Zhang"],
        "start_date": "1 Nov 2020",
        "scholarship": "Wellington Doctoral Scholarship"
    },
    {
        "id": 9,
        "name": "Junhao Huang",
        "topic": "Evolutionary Design of Deep Convolutional Neural Networks",
        "supervisors": ["Prof Bing Xue", "Prof Mengjie Zhang"],
        "start_date": "1 Nov 2020",
        "scholarship": "China Scholarship Council"
    },
    {
        "id": 10,
        "name": "Gonglin Yuan",
        "topic": "Evolutionary Neural Architecture Search for Image Classification",
        "supervisors": ["Prof Bing Xue", "Prof Mengjie Zhang"],
        "start_date": "1 July 2020",
        "scholarship": "Wellington Doctoral Scholarship"
    },
    {
        "id": 11,
        "name": "Harisu Shehu",
        "topic": "Emotion Detection",
        "supervisors": ["Prof Will Browne", "Dr Hedwig Eisenbarth"],
        "start_date": "1 Nov 2019", 
        "scholarship": "Victoria Doctoral Scholarship"
    },
    {
        "id": 12,
        "name": "Peng Wang",
        "topic": "Evolutionary Computation for Feature Selection", 
        "supervisors": ["Prof Bing Xue", "Prof Mengjie Zhang"],
        "start_date": "1 Nov 2019",
        "scholarship": "China Scholarship Council"

    }, 
    {
        "id": 13,
        "name": "Qinglan Fan",
        "topic": "Evolutionary Computation for Computer Vision",
        "supervisors": ["Prof Mengjie Zhang", "Prof Bing Xue"],
        "start_date": "1 Nov 2019", 
        "scholarship": "Victoria Doctoral Scholarship."
    },
    {
        "id": 14,
        "name": "Kosisochukwu Madukwe",
        "topic": "Text Mining",
        "supervisors": ["Prof Bing Xue", "A/Prof Xiaoying Sharon Gao"],
        "start_date": "1 May 2019",
        "scholarship": "Victoria Master Scholarship"
    },
    {
        "id": 15,
        "name": "Trevor Londt",
        "topic": "Evolutionary Computation and Deep Neural Networks for Text Mining",
        "supervisors": ["Prof Bing Xue", "A/Prof Xiaoying Sharon Gao"],
        "start_date": "1 March 2019",
        "scholarship": "Victoria Master Scholarship"
    },    
    {
        "id": 16,
        "name": "Bin Wang",
        "topic": "Evolving Deep Neural Networks for Image Classification",
        "supervisors": ["Prof Bing Xue", "Prof Mengjie Zhang"],
        "start_date": "1 March 2019",
        "scholarship": ""
    },
    {
        "id": 17,
        "name": "Ramya Anasseriyil Viswambaran",
        "topic": "Machine learning for IOT",
        "supervisors": ["Dr Aaron Chen", "Prof Bing Xue"],
        "start_date": "1 Sep 2018",
        "scholarship": ""
    },
    {
        "id": 18,
        "name": "Damien O'Neill",
        "topic": "Evolving Deep Learning",
        "supervisors": ["Prof Bing Xue", "Prof Mengjie Zhang"],
        "start_date": "1 Mar 2018",
        "scholarship": "Victoria Doctoral Scholarship"
    }
]
"#;