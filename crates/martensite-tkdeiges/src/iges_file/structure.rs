//! What is parsed from a file: the start lines, the global parameters, the directory entries and
//! the parameters of each.
use super::parameters::IgesFileParamKind;

/// One parsed parameter; its value is a C text, which ends at its first NUL.
pub(super) struct IgesFileParam {
    pub(super) kind: IgesFileParamKind,
    pub(super) value: Vec<u8>,
}

impl IgesFileParam {
    /// The value up to its terminator.
    pub(super) fn text(&self) -> &[u8] {
        c_text(&self.value)
    }
}

/// A C text out of a buffer.
pub(super) fn c_text(bytes: &[u8]) -> &[u8] {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    &bytes[..end]
}

/// One directory entry with its parameters.
pub(super) struct IgesFilePart {
    pub(super) values: [i32; 17],
    pub(super) res1: [u8; 9],
    pub(super) res2: [u8; 9],
    pub(super) nom: [u8; 9],
    pub(super) num: [u8; 9],
    pub(super) list: Vec<IgesFileParam>,
    pub(super) numpart: i32,
}

/// The list the parameters being parsed go to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum IgesFileListId {
    Start,
    Header,
    Part(usize),
}

/// What is parsed from a file.
pub(super) struct IgesFileStructure {
    pub(super) start: Vec<IgesFileParam>,
    pub(super) header: Vec<IgesFileParam>,
    pub(super) parts: Vec<IgesFilePart>,
    pub(super) nb_params: i32,
    current_list: IgesFileListId,
    current_part: Option<usize>,
    current_param: Option<(IgesFileListId, usize)>,
}

impl IgesFileStructure {
    pub(super) fn new() -> Self {
        Self {
            start: Vec::new(),
            header: Vec::new(),
            parts: Vec::new(),
            nb_params: 0,
            current_list: IgesFileListId::Start,
            current_part: None,
            current_param: None,
        }
    }
    /// Selects the list of the global section; the current parameter becomes its first one.
    pub(super) fn set_global(&mut self) {
        if self.current_list == IgesFileListId::Header {
            return;
        }
        self.current_list = IgesFileListId::Header;
        self.current_param = (!self.header.is_empty()).then_some((IgesFileListId::Header, 0));
    }
    /// Starts a directory entry, numbered in its section.
    pub(super) fn new_part(&mut self, numpart: i32) {
        let index = self.parts.len();
        self.parts.push(IgesFilePart {
            values: [0; 17],
            res1: [0; 9],
            res2: [0; 9],
            nom: [0; 9],
            num: [0; 9],
            list: Vec::new(),
            numpart,
        });
        self.current_part = Some(index);
        self.current_list = IgesFileListId::Part(index);
    }
    /// The directory entry being filled.
    pub(super) fn current_part_mut(&mut self) -> Option<&mut IgesFilePart> {
        self.current_part
            .and_then(|index| self.parts.get_mut(index))
    }
    /// Selects the directory entry a parameter record refers to: the current one, the next, or by
    /// a search. A number that is in no entry leaves the list of the next entry selected and no
    /// entry current, so what follows goes on in that list.
    pub(super) fn cur_part(&mut self, directory: i32) {
        let Some(current) = self.current_part else {
            return;
        };
        if self.parts[current].numpart == directory {
            return;
        }
        let next = if current + 1 < self.parts.len() {
            current + 1
        } else {
            0
        };
        self.current_part = Some(next);
        self.current_list = IgesFileListId::Part(next);
        if self.parts[next].numpart == directory {
            return;
        }
        match self.parts.iter().position(|part| part.numpart == directory) {
            Some(found) => {
                self.current_part = Some(found);
                self.current_list = IgesFileListId::Part(found);
            }
            None => self.current_part = None,
        }
    }
    fn list_mut(&mut self, id: IgesFileListId) -> &mut Vec<IgesFileParam> {
        match id {
            IgesFileListId::Start => &mut self.start,
            IgesFileListId::Header => &mut self.header,
            IgesFileListId::Part(index) => &mut self.parts[index].list,
        }
    }
    /// Appends a parameter to the selected list; `value` is a C text.
    pub(super) fn new_param(&mut self, kind: IgesFileParamKind, value: &[u8]) {
        let id = self.current_list;
        let list = self.list_mut(id);
        list.push(IgesFileParam {
            kind,
            value: value.to_vec(),
        });
        let index = list.len() - 1;
        self.current_param = Some((id, index));
        self.nb_params += 1;
    }
    /// Appends text to the current parameter.
    pub(super) fn add_param(&mut self, value: &[u8]) {
        if value.is_empty() {
            return;
        }
        let Some((id, index)) = self.current_param else {
            return;
        };
        let parameter = &mut self.list_mut(id)[index];
        let old_length = c_text(&parameter.value).len();
        parameter.value.truncate(old_length);
        parameter.value.extend_from_slice(value);
    }
}
