use super::*;
use crate::SessionId;
use crate::execution::{Action, Reply};

impl Node {
    pub async fn execution_on(&self, machine: &str, action: Action) -> Result<Reply, String> {
        action.validate()?;
        if let Some(reply) = self
            .via_serve(ipc::Op::ExecutionOn {
                machine: machine.into(),
                action: action.clone(),
            })
            .await
        {
            return match reply? {
                ipc::Reply::Execution { value } => Ok(value),
                ipc::Reply::Refused { why } => Err(why),
                other => Err(msg::serve_non_answer(format_args!("{other:?}"))),
            };
        }
        let (_, home) = self.resolve(machine)?;
        if home == self.device() {
            return self.execution_here(action).await;
        }
        let ep = endpoint::bind_with(
            self.secret_key().clone(),
            self.relay_map(),
            self.relays_n0(),
        )
        .await
        .map_err(|e| e.to_string())?;
        let result = self.execution_with(&ep, machine, action).await;
        ep.close().await;
        result
    }

    pub(super) async fn execution_with(
        &self,
        ep: &iroh::Endpoint,
        machine: &str,
        action: Action,
    ) -> Result<Reply, String> {
        action.validate()?;
        let (channel, home) = self.resolve(machine)?;
        if self.sat_and_sponsor(&channel)?.is_some() {
            return Err("Command tasks and terminal control require a Khor host".into());
        }
        if home == self.device() {
            return self.execution_here(action).await;
        }
        match self
            .ask(ep, &channel, &Request::Execution { action })
            .await?
        {
            Response::Execution { value } => Ok(value),
            Response::Refused { why } => Err(why),
            other => Err(msg::peer_non_answer(format_args!("{other:?}"))),
        }
    }

    pub(super) async fn execution_here(&self, action: Action) -> Result<Reply, String> {
        self.refuse_unless_hosting()?;
        let root = self.root().to_path_buf();
        tokio::task::spawn_blocking(move || crate::execution::handle(&root, action))
            .await
            .map_err(|e| e.to_string())?
    }

    pub async fn command_task_on(
        &self,
        machine: &str,
        request: crate::command_task::Request,
    ) -> Result<crate::command_task::Reply, String> {
        match self
            .execution_on(machine, Action::CommandTask(request))
            .await?
        {
            Reply::CommandTask(value) => Ok(value),
            _ => Err("Unexpected command task response".into()),
        }
    }

    pub async fn terminal_on(
        &self,
        machine: &str,
        request: crate::terminal_control::Request,
    ) -> Result<crate::terminal_control::Reply, String> {
        let target = if machine.is_empty() {
            self.every_session()?
                .into_iter()
                .find(|v| v.session.id.0 == request.session())
                .map(|v| v.session.home.hex())
                .ok_or_else(|| format!("No such session: {}", request.session()))?
        } else {
            machine.to_owned()
        };
        match self
            .execution_on(&target, Action::Terminal(request))
            .await?
        {
            Reply::Terminal(value) => Ok(value),
            _ => Err("Unexpected terminal response".into()),
        }
    }

    pub async fn ended_anywhere(
        &self,
        id: &SessionId,
    ) -> Result<Option<crate::live::Ended>, String> {
        match self
            .terminal_on(
                "",
                crate::terminal_control::Request::Ended {
                    session: id.0.clone(),
                },
            )
            .await?
        {
            crate::terminal_control::Reply::Ended { value } => Ok(value),
            _ => Err("Unexpected ended terminal response".into()),
        }
    }
}
