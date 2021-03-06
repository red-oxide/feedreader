// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under channels can be retrieved by using the methods under
//! `Channel`.


use ChannelGetters;
use rss::{Category, Channel, Cloud, Image, Item, TextInput};
use rss::extension::itunes::ITunesChannelExtension;


impl ChannelGetters for Channel
{
    /// Get the title that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let title = "The Linux Action Show! OGG";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .title(title)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(title.to_owned(), channels.title());
    /// ```
    fn title(&self) -> String
    {
        self.title.clone()
    }


    /// Get the link that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(link.to_owned(), channels.link());
    /// ```
    fn link(&self) -> String
    {
        self.link.clone()
    }


    /// Get the description that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .description(description.as_ref())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(description.to_owned(), channels.description());
    /// ```
    fn description(&self) -> String
    {
        self.description.clone()
    }


    /// Get the optional language that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let language_string = "en".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .language(Some(language_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let language_option = channels.language();
    /// assert!(language_option.is_some());
    ///
    /// assert_eq!(language_string.clone(), language_option.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .language(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.language().is_none());
    /// ```
    fn language(&self) -> Option<String>
    {
        self.language.clone()
    }


    /// Get the optional copyright that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let copyright_string =
    ///     "Copyright 2002, Spartanburg Herald-Journal".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .copyright(Some(copyright_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let copyright_option = channels.copyright();
    /// assert!(copyright_option.is_some());
    ///
    /// assert_eq!(copyright_string.clone(), copyright_option.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .copyright(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.copyright().is_none());
    /// ```
    fn copyright(&self) -> Option<String>
    {
        self.copyright.clone()
    }


    /// Get the optional managing editor that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let managing_editor_string =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .managing_editor(Some(managing_editor_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let managing_editor_option = channels.managing_editor();
    /// assert!(managing_editor_option.is_some());
    ///
    /// assert_eq!(managing_editor_string.clone(),
    /// managing_editor_option.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .managing_editor(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.managing_editor().is_none());
    /// ```
    fn managing_editor(&self) -> Option<String>
    {
        self.managing_editor.clone()
    }

    /// Get the optional web master that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let webmaster_string =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .webmaster(Some(webmaster_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let webmaster_option = channels.webmaster();
    /// assert!(webmaster_option.is_some());
    ///
    /// assert_eq!(webmaster_string.clone(), webmaster_option.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .webmaster(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.webmaster().is_none());
    /// ```
    fn webmaster(&self) -> Option<String>
    {
        self.webmaster.clone()
    }


    /// Get the optional pub date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .pub_date(Some(pub_date.to_owned()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let local = channels.pub_date();
    /// assert!(local.is_some());
    ///
    /// assert_eq!(pub_date.to_owned(), local.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .pub_date(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.pub_date().is_none());
    /// ```
    fn pub_date(&self) -> Option<String>
    {
        self.pub_date.clone()
    }


    /// Get the optional last build date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let last_build_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .last_build_date(Some(last_build_date.to_owned()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let local = channels.last_build_date();
    /// assert!(local.is_some());
    ///
    /// assert_eq!(last_build_date.to_owned(), local.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .last_build_date(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.last_build_date().is_none());
    /// ```
    fn last_build_date(&self) -> Option<String>
    {
        self.last_build_date.clone()
    }


    /// Get the categories that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters, CategoryBuilder};
    ///
    /// let category_1 = CategoryBuilder::new()
    ///     .domain(None)
    ///     .name("Media")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let category_2 = CategoryBuilder::new()
    ///     .domain(Some("http://jupiterbroadcasting.com".to_owned()))
    ///     .name("Podcast")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories_vec = vec![category_1, category_2];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .categories(categories_vec.clone())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories = channels.categories();
    /// assert!(!categories.is_empty());
    ///
    /// assert_eq!(categories_vec.clone().len(), categories.len());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .categories(Vec::new())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.categories().is_empty());
    /// ```
    fn categories(&self) -> Vec<Category>
    {
        self.categories.clone()
    }


    /// Get the optional generator that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let generator_string = "Feeder 2.5.12(2294); ".to_owned()
    /// + "Mac OS X Version 10.9.5 (Build 13F34) "
    /// + "http://reinventedsoftware.com/feeder/";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .generator(Some(generator_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let generator_option = channels.generator();
    /// assert!(generator_option.is_some());
    ///
    /// assert_eq!(generator_string.clone(), generator_option.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .generator(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.generator().is_none());
    /// ```
    fn generator(&self) -> Option<String>
    {
        self.generator.clone()
    }


    /// Get the optional docs that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let docs_string = "http://blogs.law.harvard.edu/tech/rss/".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .docs(Some(docs_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let docs_option = channels.docs();
    /// assert!(docs_option.is_some());
    ///
    /// assert_eq!(docs_string.clone(), docs_option.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .docs(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.docs().is_none());
    /// ```
    fn docs(&self) -> Option<String>
    {
        self.docs.clone()
    }

    /// Get the optional cloud that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters, CloudBuilder};
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain("http://rpc.sys.com/")
    ///     .port(80)
    ///     .path("/RPC2")
    ///     .register_procedure("pingMe")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .cloud(Some(cloud))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.cloud().is_some());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .cloud(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.cloud().is_none());
    /// ```
    fn cloud(&self) -> Option<Cloud>
    {
        self.cloud.clone()
    }


    /// Get the optional ttl that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let ttl_num = 60;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .ttl(Some(ttl_num))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let ttl_option = channels.ttl();
    /// assert!(ttl_option.is_some());
    ///
    /// assert_eq!(ttl_num.to_string(), ttl_option.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .ttl(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.ttl().is_none());
    /// ```
    fn ttl(&self) -> Option<String>
    {
        self.ttl.clone()
    }


    /// Get the optional image that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters, ImageBuilder};
    ///
    /// let image = ImageBuilder::new()
    ///     .link("http://www.jupiterbroadcasting.com")
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .height(None)
    ///     .width(None)
    ///     .description(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .image(Some(image))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.image().is_some());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .image(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.image().is_none());
    /// ```
    fn image(&self) -> Option<Image>
    {
        self.image.clone()
    }


    /// Get the optional rating that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .rating(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.rating().is_none());
    /// ```
    fn rating(&self) -> Option<String>
    {
        None
    }


    /// Get the optional text input that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters, TextInputBuilder};
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .title("Enter Comment")
    ///     .description("Provided Feedback")
    ///     .name("Comment")
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .text_input(Some(text_input))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.text_input().is_some());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .text_input(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.text_input().is_none());
    /// ```
    fn text_input(&self) -> Option<TextInput>
    {
        self.text_input.clone()
    }

    /// Get the skip hours that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let skip_hours_vec: Vec<i64> = vec![6,7,8,14,22];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_hours(skip_hours_vec.clone())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let skip_hours  = channels.skip_hours();
    /// assert!(!skip_hours.is_empty());
    ///
    /// let len = skip_hours_vec.clone().len();
    /// assert_eq!(len, skip_hours.len());
    ///
    /// for x in 0..len {
    ///     assert_eq!(skip_hours_vec[x].to_string(), skip_hours[x]);
    /// }
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_hours(Vec::new())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.skip_hours().is_empty());
    /// ```
    fn skip_hours(&self) -> Vec<String>
    {
        self.skip_hours.clone()
    }


    /// Get the skip days that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let skip_days_vec: Vec<String> = vec!["Monday".to_owned(),
    /// "Sunday".to_owned(), "Thursday".to_owned(),
    ///     "Wednesday".to_owned()];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_days(skip_days_vec.clone())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let skip_days = channels.skip_days();
    /// assert!(!skip_days.is_empty());
    ///
    /// let len = skip_days_vec.clone().len();
    /// assert_eq!(len, skip_days.len());
    ///
    /// for x in 0..len {
    ///     assert_eq!(skip_days_vec[x], skip_days[x].clone());
    /// }
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_days(Vec::new())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.skip_days().is_empty());
    /// ```
    fn skip_days(&self) -> Vec<String>
    {
        self.skip_days.clone()
    }


    /// Get the items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters, ItemBuilder};
    ///
    /// let link = "http://www.jupiterbroadcasting.com/97561/".to_owned()
    /// + "making-music-with-linux-las-408/";
    ///
    /// let description = "<![CDATA[<p>In special Rasberry Pi 3 ".to_owned()
    /// + "edition of the show we look at the new hardware, review & chat with "
    /// + "Mycroft CTO Ryan Sipes on how important the Raspberry Pi is for "
    /// + "development of their open artificial intelligence platform & get "
    /// + "the latest news.</p><p>Plus replacing Spotify on Linux, the new "
    /// + "Microsoft lock-in, our hosts face a moral quandary & more!</p>]]>";
    ///
    /// let title = "Making Music with Linux | LAS 408".to_owned();
    ///
    /// let item_1 = ItemBuilder::new()
    ///     .title(Some(title))
    ///     .link(Some(link))
    ///     .description(None)
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item_2 = ItemBuilder::new()
    ///     .title(None)
    ///     .link(None)
    ///     .description(Some(description))
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let items_vec = vec![item_1, item_2];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .items(items_vec.clone())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let items = channels.items();
    /// assert!(!items.is_empty());
    ///
    /// assert_eq!(items_vec.clone().len(), items.len());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .items(Vec::new())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.items().is_empty());
    /// ```
    fn items(&self) -> Vec<Item>
    {
        self.items.clone()
    }


    /// Get the optional `ITunesChannelExtension` under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesOwnerBuilder, ITunesCategoryBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_owned()))
    ///     .name(Some("name".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Some(Box::new(subcategory)))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories = vec![category];
    ///
    /// let itunes_channel = ITunesChannelExtensionBuilder::new()
    ///     .author(Some("author".to_owned()))
    ///     .block(Some("block".to_owned()))
    ///     .image(Some("image".to_owned()))
    ///     .explicit(Some("explicit".to_owned()))
    ///     .subtitle(Some("subtitle".to_owned()))
    ///     .summary(Some("summary".to_owned()))
    ///     .keywords(Some("keywords".to_owned()))
    ///     .new_feed_url(Some("new_feed_url".to_owned()))
    ///     .complete(Some("complete".to_owned()))
    ///     .owner(Some(owner))
    ///     .categories(categories)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channel = ChannelBuilder::new()
    ///     .itunes_ext(Some(itunes_channel))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channel.itunes_ext().is_some());
    /// ```
    ///
    /// ```
    /// use feed::{ChannelBuilder, ChannelGetters};
    ///
    /// let channel = ChannelBuilder::new()
    ///     .itunes_ext(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channel.itunes_ext().is_none());
    /// ```
    fn itunes_ext(&self) -> Option<ITunesChannelExtension>
    {
        self.itunes_ext.clone()
    }
}
