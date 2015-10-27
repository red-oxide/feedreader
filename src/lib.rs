// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate rss;
extern crate url;
extern crate curl;

use curl::http;
use url::Url;
use rss::{Rss, Category};

use std::str;

/// Retrieve a String containing the rss feed from an URL
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// ```
pub fn new(url: &str) -> String {
    let feed_url = Url::parse(url).unwrap();

    let resp = http::handle().get(&feed_url).exec().unwrap();
    let body = resp.get_body();

    println!("body={:?}", body);

    let feed = str::from_utf8(body).unwrap().to_string();
    feed
}

/// Retrieve a String containing channel title
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_title = feed-reader::get_channel_title(&feed);
/// ```
pub fn get_channel_title(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let title = rss_feed.title;
            println!("Channel Title: {:?}", title);
            return title;
        }
    }
}

/// Retrieve a String containing channel language
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_language = feed-reader::get_channel_language(&feed);
/// ```
pub fn get_channel_language(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let language = rss_feed.language.unwrap();
            println!("Channel Language: {:?}", language);
            return language;
        }
    }
}

/// Retrieve a String containing channel copyright
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_copyright = feed-reader::get_channel_copyright(&feed);
/// ```
pub fn get_channel_copyright(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let copyright = rss_feed.copyright.unwrap();
            println!("Channel Copyright: {:?}", copyright);
            return copyright;
        }
    }
}

/// Retrieve a String containing channel managing editor
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_managing_editor = feed-reader::get_channel_managing_editor(&feed);
/// ```
pub fn get_channel_managing_editor(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let managing_editor = rss_feed.managing_editor.unwrap();
            println!("Channel Managing Editor: {:?}", managing_editor);
            return managing_editor;
        }
    }
}

/// Retrieve a String containing channel web master
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_web_master = feed-reader::get_channel_web_master(&feed);
/// ```
pub fn get_channel_web_master(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let web_master = rss_feed.web_master.unwrap();
            println!("Channel Web Master: {:?}", web_master);
            return web_master;
        }
    }
}

/// Retrieve a String containing channel last build date
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_last_build_date = feed-reader::get_channel_last_build_date(&feed);
/// ```
pub fn get_channel_last_build_date(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let last_build_date = rss_feed.last_build_date.unwrap();
            println!("Channel Last Build Date: {:?}", last_build_date);
            return last_build_date;
        }
    }
}

/// Retrieve a Vec<Category> containing channel Rss Categories
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_categories = feed-reader::get_channel_categories(&feed);
/// ```
pub fn get_channel_categories(feed: String) -> Vec<Category> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let categories = rss_feed.categories;
            println!("Channel Categories: {:?}", categories);
            return categories;
        }
    }
}

/// Retrieve a String containing channel generator
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_generator = feed-reader::get_channel_generator(&feed);
/// ```
pub fn get_channel_generator(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let generator = rss_feed.generator.unwrap();
            println!("Channel Generator: {:?}", generator);
            return generator;
        }
    }
}

/// Retrieve a String containing channel docs
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_docs = feed-reader::get_channel_docs(&feed);
/// ```
pub fn get_channel_docs(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let docs = rss_feed.docs.unwrap();
            println!("Channel Docs: {:?}", docs);
            return docs;
        }
    }
}

/// Retrieve a String containing channel ttl
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_ttl = feed-reader::get_channel_ttl(&feed);
/// ```
pub fn get_channel_ttl(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let ttl = rss_feed.ttl.unwrap();
            println!("Channel ttl: {:?}", ttl);
            return ttl;
        }
    }
}

/// Retrieve a String containing channel image
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_image = feed-reader::get_channel_image(&feed);
/// ```
pub fn get_channel_image(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let image = rss_feed.image.unwrap();
            println!("Channel Image: {:?}", image);
            return image;
        }
    }
}

/// Retrieve a String containing channel rating
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_ratingr = feed-reader::get_channel_rating(&feed);
/// ```
pub fn get_channel_rating(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let rating = rss_feed.rating.unwrap();
            println!("Channel Rating: {:?}", rating);
            return rating;
        }
    }
}

/// Retrieve a String containing channel skip hours
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_skip_hours = feed-reader::get_channel_skip_hours(&feed);
/// ```
pub fn get_channel_skip_hours(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let skip_hours = rss_feed.skip_hours.unwrap();
            println!("Channel Skip Hours: {:?}", skip_hours);
            return skip_hours;
        }
    }
}

/// Retrieve a String containing channel skip_days
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let channel_skip_days = feed-reader::get_channel_skip_days(&feed);
/// ```
pub fn get_channel_skip_days(feed: String) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let skip_days = rss_feed.skip_hours.unwrap();
            println!("Channel Skip Days: {:?}", skip_days);
            return skip_days;
        }
    }
}

/// Retrieve a String containing item title
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_title = feed-reader::get_item_title(&feed, item_element);
/// ```
pub fn get_item_title(feed: String, element: usize) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let title = item.title.unwrap();
            println!("Item Title: {:?}", title);
            return title;
        }
    }
}

/// Retrieve a String containing item link
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_link = feed-reader::get_item_link(&feed, item_element);
/// ```
pub fn get_item_link(feed: String, element: usize) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let link = item.link.unwrap();
            println!("Item Link: {:?}", link);
            return link;
        }
    }
}

/// Retrieve a String containing item description
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_description = feed-reader::get_item_description(&feed, item_element);
/// ```
pub fn get_item_description(feed: String, element: usize) -> String
{
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let description = item.description.unwrap();
            println!("Item Description: {:?}", description);
            return description;
        }
    }
}

/// Retrieve a String containing item author
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_author = feed-reader::get_item_title(&feed, item_element);
/// ```
pub fn get_item_author(feed: String, element: usize) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let author = item.author.unwrap();
            println!("Item Author: {:?}", author);
            return author;
        }
    }
}

/// Retrieve a Vec<Category> containing item Rss Categories
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_categories = feed-reader::get_item_categories(&feed, item_element);
/// ```
pub fn get_item_categories(feed: String, element: usize) -> Vec<Category> {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let categories = item.categories;
            println!("Item Categories: {:?}", categories);
            return categories;
        }
    }
}

/// Retrieve a String containing item comments
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_comments = feed-reader::get_item_comments(&feed, item_element);
/// ```
pub fn get_item_comments(feed: String, element: usize) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let comments = item.comments.unwrap();
            println!("Item Comments: {:?}", comments);
            return comments;
        }
    }
}

/// Retrieve a String containing item pub date
///
/// # Example
///
/// ```
/// extern crate feed-reader;
///
/// use feed-reader::*;
///
/// let feed = feed-reader::new("http://feeds2.feedburner.com/TheLinuxActionShowOGG");
/// let item_element: usize = 1;
/// let item_pub_date = feed-reader::get_item_pub_date(&feed, item_element);
/// ```
pub fn get_item_pub_date(feed: String, element: usize) -> String {
    match feed.parse::<Rss>().unwrap() {
        Rss(rss_feed) => {
            let item = rss_feed.items[element].clone();
            let pub_date = item.pub_date.unwrap();
            println!("Item Pub Date: {:?}", pub_date);
            return pub_date;
        }
    }
}
