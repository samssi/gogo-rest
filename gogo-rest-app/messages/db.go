package messages

import (
	"context"
	"gogo-rest-app/db"
	"log"
)

type dbMessage struct {
	messageId int
	message   string
}

func deleteMessage(messageId int) {
	_, err := db.Pool.Exec(context.Background(), "delete from message where message_id = $1", messageId)
	if err != nil {
		log.Printf("Deletion exec failed: %v\n", err)
	}
}

func popMessage() *dbMessage {
	var message dbMessage
	if err := db.Pool.QueryRow(context.Background(), "select message_id, message from message order by message_id").Scan(&message.messageId, &message.message); err != nil {
		return nil
	}

	deleteMessage(message.messageId)

	return &message
}

func insertMessage(message message) {
	log.Printf("Adding message to the db: %v\n", message.Message)

	_, err := db.Pool.Exec(context.Background(), "insert into message (message) values ($1)", message.Message)
	if err != nil {
		log.Printf("Insert exec failed: %v\n", err)
	}
}
