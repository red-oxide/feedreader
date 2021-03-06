// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under cloud can be retrieved by using the methods under `Cloud`.


use CloudGetters;
use rss::Cloud;


impl CloudGetters for Cloud
{
    /// Get the domain that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{CloudBuilder, CloudGetters};
    ///
    /// let domain = "http://rpc.sys.com/";
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain(domain)
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(domain.to_owned(), cloud.domain());
    /// ```
    fn domain(&self) -> String
    {
        self.domain.clone()
    }


    /// Get the port that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{CloudBuilder, CloudGetters};
    ///
    /// let port: i64 = 80;
    ///
    /// let cloud = CloudBuilder::new()
    ///     .port(port)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(port.to_string(), cloud.port());
    /// ```
    fn port(&self) -> String
    {
        self.port.clone()
    }


    /// Get the path that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{CloudBuilder, CloudGetters};
    ///
    /// let path = "/RPC2";
    ///
    /// let cloud = CloudBuilder::new()
    ///     .path(path)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(path.to_owned(), cloud.path());
    /// ```
    fn path(&self) -> String
    {
        self.path.clone()
    }


    /// Get the register procedure that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{CloudBuilder, CloudGetters};
    ///
    /// let register_procedure = "pingMe";
    /// let cloud = CloudBuilder::new()
    ///     .register_procedure(register_procedure)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(register_procedure.to_owned(), cloud.register_procedure());
    /// ```
    fn register_procedure(&self) -> String
    {
        self.register_procedure.clone()
    }


    /// Get the protocol that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{CloudBuilder, CloudGetters};
    ///
    /// let protocol = "soap";
    ///
    /// let cloud = CloudBuilder::new()
    ///     .protocol(protocol)
    ///     .domain("http://rpc.sys.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(protocol.to_owned(), cloud.protocol());
    /// ```
    fn protocol(&self) -> String
    {
        self.protocol.clone()
    }
}
