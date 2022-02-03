use std::fmt;

use elem::RQElem;

use crate::elem::{anonymous::Anonymous, reply::Reply};
use crate::pb::msg;
use crate::pb::msg::elem::Elem;

pub mod elem;
pub mod fragment;

#[derive(Debug, Default, Clone)]
pub struct MessageChain(pub Vec<msg::elem::Elem>);

impl MessageChain {
    pub fn push<E: Into<Vec<msg::elem::Elem>>>(&mut self, e: E) {
        self.0.extend(e.into())
    }

    pub fn anonymous(&self) -> Option<Anonymous> {
        self.0
            .iter()
            .filter_map(|e| match e {
                msg::elem::Elem::AnonGroupMsg(anonymous) => {
                    Some(Anonymous::from(anonymous.clone()))
                }
                _ => None,
            })
            .next()
    }

    pub fn reply(&self) -> Option<Reply> {
        self.0
            .iter()
            .filter_map(|e| match e {
                msg::elem::Elem::SrcMsg(src_msg) => Some(Reply::from(src_msg.clone())),
                _ => None,
            })
            .next()
    }

    pub fn with_anonymous(&mut self, anonymous: Anonymous) {
        self.0.insert(0, msg::elem::Elem::from(anonymous))
    }

    pub fn with_reply(&mut self, reply: Reply) {
        let index = if let Some(_) = self.anonymous() { 1 } else { 0 };
        self.0.insert(index, msg::elem::Elem::from(reply))
    }
}

impl IntoIterator for MessageChain {
    type Item = RQElem;
    type IntoIter = impl Iterator<Item = RQElem> + 'static;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .filter_map(|e| match e {
                Elem::SrcMsg(_) => None,
                Elem::AnonGroupMsg(_) => None,
                _ => Some(e),
            })
            .map(RQElem::from)
    }
}

impl From<Vec<msg::Elem>> for MessageChain {
    fn from(elements: Vec<msg::Elem>) -> Self {
        Self(elements.into_iter().filter_map(|e| e.elem).collect())
    }
}

impl From<MessageChain> for Vec<msg::Elem> {
    fn from(e: MessageChain) -> Self {
        e.0.into_iter()
            .map(|e| msg::Elem { elem: Some(e) })
            .collect()
    }
}

impl fmt::Display for MessageChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.clone().into_iter() {
            fmt::Display::fmt(&x, f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut chain = MessageChain::default();
        chain.push(elem::text::Text::new("hello".into()));
        for e in chain.into_iter() {
            println!("{:?}", e)
        }
    }

    #[test]
    fn test_display() {
        let mut chain = MessageChain::default();
        chain.with_anonymous(elem::anonymous::Anonymous::default());
        chain.with_reply(elem::reply::Reply::default());
        chain.push(elem::text::Text::new("hello".into()));
        chain.push(elem::at::At::new(12345));
        chain.push(elem::text::Text::new("world".into()));
        chain.push(elem::face::Face::new(1));
        chain.push(elem::market_face::Dice::new(1));
        chain.push(elem::market_face::FingerGuessing::Rock);
        chain.push(elem::market_face::MarketFace {
            name: "xx".into(),
            ..Default::default()
        });
        println!("{}", chain);
        println!("{:?}", chain.reply());
        println!("{:?}", chain.anonymous());
    }
}
