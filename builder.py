import getopt
import os
import sys

USAGE = f"Usage : python {sys.argv[0]} [-h, --help] [-d, --dir <target_dir>] [-f, --force] <usecase> [<usecase>, ...]\n" \
        + "    -h, --help               Print command details\n" \
        + "    -d, --dir <target_dir>   Specify the target dir to use (create it if not exists)\n" \
        + "                             Default value is current directory\n" \
        + "    -f, --force              Force overwriting existing use case\n" \
        + "    -p, --project-name       Project name (use for Cargo initialization if not exists) in snake_case\n"


def main():
    try:
        project_name, root_dir, force_overwriting, use_cases = parse_args()

        if force_overwriting:
            response = input("Do you really want to overwrite use cases (Y/n) : ")
            if response.lower() == 'n':
                return

        check_upper_case(project_name, "Project name")

        target_path = os.path.join(root_dir, project_name)

        init_cargo(target_path)

        for use_case in use_cases:
            check_upper_case(use_case, "Use case")
            check_use_case_in_readme(target_path, use_case, force_overwriting)
            add_use_case_to_readme(target_path, use_case)

        init_domain(target_path)
        init_infrastructure(target_path)

        for use_case in use_cases:
            init_business(target_path, use_case)
            init_application(target_path, use_case)

    except UsageError as error:
        print(f"ERROR - {error}\n")
        raise SystemExit(USAGE)


def parse_args():
    options, arguments = getopt.getopt(sys.argv[1:], "hp:d:f", ["help", "project-name=", "dir=", "force"])

    project_name = ""
    target_dir = os.getcwd()
    force_overwriting = False
    for opt, arg in options:
        if opt in ("-h", "--help"):
            print(USAGE)
            sys.exit(0)

        if opt in ("-p", "--project-name"):
            project_name = arg

        if opt in ("-d", "--dir"):
            target_dir = os.path.abspath(arg)

        if opt in ("-f", "--force"):
            force_overwriting = True

    if not project_name:
        UsageError("No project name found")

    if not arguments:
        raise UsageError("No use cases found")

    return project_name, target_dir, force_overwriting, arguments


def check_upper_case(string, element):
    for char in string:
        if char.isupper():
            raise UsageError(f'{element} must be in snake_case')


def check_use_case_in_readme(target_path, use_case, force_overwriting):
    file = open(os.path.join(target_path, "README.md"), 'r')
    if use_case in file.read() and not force_overwriting :
        raise Exception(f'Use case {use_case} already exists.\nUse "-f" to overwrite existing files.')


def add_use_case_to_readme(target_path, use_case):
    print(f'Adding "{use_case}" to file "{os.path.join(target_path, "README.md")}"')
    file_with_content(os.path.join(target_path, "README.md"), [f'{use_case} : Explain here the goal', ''], 'a')


def init_cargo(target_path):
    if not os.path.exists(target_path):
        os.mkdir(target_path)

    domain_path = os.path.join(target_path, "domain")
    if not os.path.exists(domain_path):
        os.system(f'cargo new --lib {domain_path}')

    business_path = os.path.join(target_path, "business")
    if not os.path.exists(business_path):
        os.system(f'cargo new --lib {business_path}')

        file_with_content(os.path.join(business_path, "Cargo.toml"), ['domain = { path = "../domain" }', ''], 'a')

        file_with_content(
            os.path.join(business_path, "src", "lib.rs"),
            [
                'use std::error::Error;',
                '',
                'pub trait InputMessage {}',
                '',
                'pub trait OutputMessage {}',
                '',
                'pub trait InputBoundary<T> where T: InputMessage {',
                '   fn execute(&mut self, message: T);',
                '}',
                '',
                'pub trait OutputBoundary<T, V> where T: OutputMessage, V: Error {',
                '   fn success(&mut self, message: T);',
                '   fn error(&mut self, message: T, error: V);',
                '}',
            ],
            'w'
        )

    infrastructure_path = os.path.join(target_path, "infrastructure")
    if not os.path.exists(infrastructure_path):
        os.system(f'cargo new --lib {infrastructure_path}')

        file_with_content(
            os.path.join(infrastructure_path, "Cargo.toml"),
            ['domain = { path = "../domain" }', 'business = { path = "../business" }', ''],
            'a'
        )

    application_path = os.path.join(target_path, "application")
    if not os.path.exists(application_path):
        os.system(f'cargo new --lib {application_path}')

        file_with_content(
            os.path.join(application_path, "Cargo.toml"),
            [
                'domain = { path = "../domain" }',
                'business = { path = "../business" }',
                'infrastructure = { path = "../infrastructure" }',
                ''
            ],
            'a'
        )

        file_with_content(
            os.path.join(application_path, "src", "view_models.rs"),
            [
                'pub trait ViewModel {',
                '    fn is_success(&self) -> bool;',
                '    fn is_error(&self) -> bool;',
                '}',
            ],
            'w'
        )

    workspace_cargo = os.path.join(target_path, "Cargo.toml")
    if not os.path.exists(workspace_cargo):
        file_with_content(workspace_cargo,
                          [
                              "[workspace]",
                              "members = [",
                              '    "application",',
                              '    "business",',
                              '    "domain",',
                              '    "infrastructure",',
                              "]"],
                          'w'
                          )

    workspace_readme = os.path.join(target_path, "README.md")
    if not os.path.exists(workspace_readme):
        file_with_content(workspace_readme,
                          [
                              '# Title of your application',
                              '',
                              '## Use cases',
                              '',
                          ],
                          'w'
                          )


def init_domain(target_path):
    domain_path = os.path.join(target_path, "domain", "src")

    create_directory(os.path.join(domain_path, "entities"))
    create_directory(os.path.join(domain_path, "errors"))
    create_directory(os.path.join(domain_path, "ports"))

    file_with_content(
        os.path.join(domain_path, "lib.rs"),
        ["pub mod entities;", "pub mod errors;", "pub mod ports;"],
        'w'
    )
    file_with_content(
        os.path.join(domain_path, "entities.rs"),
        ["// Put all business entities struct in the folder with same name"],
        'w'
    )
    file_with_content(
        os.path.join(domain_path, "errors.rs"), ["// Put all business errors struct in the folder with same name"],
        'w'
    )
    file_with_content(
        os.path.join(domain_path, "ports.rs"),
        ["// Put all external dependencies abstraction in the folder with same name"],
        'w'
    )


def init_business(target_path, use_case):
    use_case_camel_case = to_camel_case(use_case)
    use_case_input_message = f'{use_case}_input_message'
    use_case_interactor = f'{use_case}_interactor'
    use_case_output_message = f'{use_case}_output_message'
    business_path = os.path.join(target_path, "business", "src")
    use_case_path = os.path.join(business_path, f'{use_case}_use_case')

    create_directory(use_case_path)

    file_with_content(
        os.path.join(business_path, "lib.rs"), [f'pub mod {use_case}_use_case;'], 'a'
    )
    file_with_content(
        os.path.join(business_path, f'{use_case}_use_case.rs'),
        [
            f'pub mod {use_case_input_message};',
            f'pub mod {use_case_interactor};',
            f'pub mod {use_case_output_message};'
        ],
        'w'
    )
    file_with_content(
        os.path.join(use_case_path, f'{use_case_input_message}.rs'),
        [
            'use crate::InputMessage;',
            '',
            f'pub struct {use_case_camel_case}InputMessage {{}}',
            '',
            f'impl {use_case_camel_case}InputMessage {{',
            '   pub fn new() -> Self { Self{} }',
            '}',
            '',
            f'impl InputMessage for {use_case_camel_case}InputMessage {{}}',
        ],
        'w'
    )
    file_with_content(
        os.path.join(use_case_path, f'{use_case_interactor}.rs'),
        [
            'use crate::{InputBoundary, OutputBoundary};',
            f'use crate::{use_case}_use_case::{use_case}_input_message::{use_case_camel_case}InputMessage;',
            f'use crate::{use_case}_use_case::{use_case}_output_message::{use_case_camel_case}OutputMessage;',
            '',
            f'pub struct {use_case_camel_case}Interactor {{',
            '   // Change generic exception with a specific exception',
            f'  presenter: Box<dyn OutputBoundary<{use_case_camel_case}OutputMessage, std::fmt::Error>>,',
            f'  output_message: {use_case_camel_case}OutputMessage,',
            '   // Add repositories : my_repository: Box<dyn MyPort>,',
            '}',
            '',
            f'impl {use_case_camel_case}Interactor {{'
            f'   pub fn new(presenter: Box<dyn OutputBoundary<{use_case_camel_case}OutputMessage, std::fmt::Error>>) -> Self {{',
            '       Self {',
            '           presenter,',
            f'          output_message: {use_case_camel_case}OutputMessage::new(),',
            '           // Add repositories',
            '       }',
            '   }',
            '}',
            '',
            f'impl InputBoundary<{use_case_camel_case}InputMessage> for {use_case_camel_case}Interactor {{',
            f'  fn execute(&mut self, message: {use_case_camel_case}InputMessage) {{',
            '       // Business processing',
            '       // If error : self.presenter.error(self.output_message.clone(), e);',
            '       self.presenter.success(self.output_message.clone());',
            '   }',
            '}',
        ],
        'w'
    )
    file_with_content(
        os.path.join(use_case_path, f'{use_case_output_message}.rs'),
        [
            '// use std::rc::Rc;',
            'use crate::OutputMessage;',
            '',
            '#[derive(Clone)]',
            f'pub struct {use_case_camel_case}OutputMessage {{',
            '   // my_entity: Option<Rc<MyEntity>>',
            '}',
            '',
            f'impl {use_case_camel_case}OutputMessage {{',
            '   pub fn new() -> Self {',
            '       Self {}',
            '   }',
            '}',
            '',
            f'impl OutputMessage for {use_case_camel_case}OutputMessage {{}}',
        ],
        'w'
    )


def init_infrastructure(target_path):
    infrastructure_path = os.path.join(target_path, "infrastructure", "src")

    create_directory(os.path.join(infrastructure_path, "repositories"))

    file_with_content(
        os.path.join(infrastructure_path, "lib.rs"), ["pub mod repositories;"],
        'w'
    )
    file_with_content(
        os.path.join(infrastructure_path, "repositories.rs"),
        ["// Put all external dependencies logic in the folder with same name"],
        'w'
    )


def init_application(target_path, use_case):
    use_case_camel_case = to_camel_case(use_case)
    use_case_presenter = f'{use_case}_presenter'
    use_case_view_model = f'{use_case}_view_model'
    application_path = os.path.join(target_path, "application", "src")
    presenters_path = os.path.join(application_path, "presenters")
    view_models_path = os.path.join(application_path, "view_models")

    create_directory(presenters_path)
    create_directory(view_models_path)

    file_with_content(
        os.path.join(application_path, "lib.rs"), [f'pub mod presenters;', f'pub mod view_models;'], 'a'
    )
    file_with_content(
        os.path.join(application_path, "presenters.rs"),
        [f'pub mod {use_case_presenter};'],
        'a'
    )
    file_with_content(
        os.path.join(application_path, "view_models.rs"),
        [f'pub mod {use_case_view_model};'],
        'a'
    )
    file_with_content(
        os.path.join(view_models_path, f'{use_case_view_model}.rs'),
        [
            'use std::rc::Rc;',
            f'use crate::view_models::ViewModel;',
            f'pub struct {use_case_camel_case}ViewModel {{',
            '    // my_entity: Option<Rc<MyEntity>>,',
            f'    error: Option<std::fmt::Error>, // Change generic error by a specific error',
            '}',
            '',
            f'impl {use_case_camel_case}ViewModel {{',
            f'  pub fn new(error: Option<std::fmt::Error>) -> Self {{',
            '       Self {',
            '           // my_entity,',
            '           error,',
            '       }',
            '   }',
            f'  pub fn get_error(&self) -> Option<&std::fmt::Error> {{',
            '       self.error.as_ref()',
            '   }',
            '}',
            '',
            f'impl ViewModel for {use_case_camel_case}ViewModel {{'
            '   fn is_success(&self) -> bool {',
            '       self.error.is_none()',
            '   }',
            '   fn is_error(&self) -> bool {',
            '       self.error.is_some()',
            '   }',
            '}',
        ],
        'a'
    )
    file_with_content(
        os.path.join(presenters_path, f'{use_case_presenter}.rs'),
        [
            f'use business::{use_case}_use_case::{use_case}_output_message::{use_case_camel_case}OutputMessage;',
            'use business::OutputBoundary;',
            f'use crate::view_models::{use_case}_view_model::{use_case_camel_case}ViewModel;',
            '',
            f'pub struct {use_case_camel_case}Presenter {{',
            f'  view_model: Option<{use_case_camel_case}ViewModel>',
            '}',
            '',
            f'impl {use_case_camel_case}Presenter {{',
            '   pub fn new() -> Self {',
            '       Self { view_model: None }',
            '   }',
            '}',
            '',
            f'impl OutputBoundary<{use_case_camel_case}OutputMessage, std::fmt::Error> for {use_case_camel_case}Presenter {{',
            f'  fn success(&mut self, message: {use_case_camel_case}OutputMessage) {{',
            '       // first argument may be : message.get_my_entity()'
            f'      let view_model = {use_case_camel_case}ViewModel::new(, None);',
            '       self.view_model = Some(view_model);',
            '   }',
            '',
            f'  fn error(&mut self, message: {use_case_camel_case}OutputMessage, error: std::fmt::Error) {{',
            '       // first argument may be : message.get_my_entity()'
            f'      let view_model = {use_case_camel_case}ViewModel::new(Some(error));',
            '       self.view_model = Some(view_model);',
            '   }',
            '}',
        ],
        'a'
    )


def create_directory(dir_path):
    if not os.path.exists(dir_path):
        os.mkdir(dir_path)


def file_with_content(file_path, content, mode):
    file = open(file_path, mode)
    for line in content:
        file.write(f'{line}\n')
    file.flush()
    file.close()


def to_camel_case(snake_case):
    return ''.join(word.title() for word in snake_case.split('_'))


class UsageError(Exception):
    pass


if __name__ == "__main__":
    main()
